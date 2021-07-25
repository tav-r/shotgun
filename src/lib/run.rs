use super::url::parse::get_params;
use super::analyze::reflection::check_response;
use clap::{Arg, App};

use std::io::{BufReader,BufRead,stdin};
use std::error::Error;

pub async fn from_cli() -> Result<(), Box<dyn Error>> {
    let matches = App::new("shotgun")
        .about("Read URLs from stdin and check for reflected parameters in responses")
        .version("0.1")
        .arg(Arg::with_name("cookie-string")
            .short("j")
            .long("cookie-string")
            .value_name("key1=value1; key2=value2; ...")
            .help("Set a cookie string for GET requests")
            .takes_value(true))
        .arg(Arg::with_name("picky")
            .short("p")
            .long("picky")
            .help("Only show matches where only the value is reflected and not 'key=value'")
        )
        .get_matches();

    run_from_stdin(matches.value_of("cookie-string").unwrap_or(""), matches.is_present("picky")).await
}

async fn run_from_stdin(cookie_string: &str, picky: bool) -> Result<(), Box<dyn Error>> {
    for line in BufReader::new(stdin()).lines().into_iter()
        .map(|l| l.unwrap())
        .filter(
            |u| u.contains("?") && (u.starts_with("http://") || u.starts_with("https://"))
        )
    {
        check_response(&line, &get_params(&line), cookie_string, picky).await?
    }

    Ok(())
}