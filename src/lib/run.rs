use super::analyze::{reflection::check_response,AnalyzeOptions};
use clap::{Arg, App};
use url::Url;

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
            .help("Only report matches where the value not (only) reflected as part of the whole URL")
        )
        .arg(Arg::with_name("script-block")
            .short("s")
            .long("script-block")
            .help("Only report matches inside HTML <script>-blocks")
        )
        .get_matches();

    let options = AnalyzeOptions{
        picky: matches.is_present("picky"),
        script_block: matches.is_present("script-block")
    };

    run_from_stdin(matches.value_of("cookie-string").unwrap_or(""), &options).await
}

async fn run_from_stdin(cookie_string: &str, options: &AnalyzeOptions) -> Result<(), Box<dyn Error>> {
    for url in BufReader::new(stdin()).lines().into_iter()
    {
        if let Ok(mut url_parsed) = Url::parse(&url.unwrap()) {
            check_response(&mut url_parsed, cookie_string, &options).await?
        }
    }

    Ok(())
}