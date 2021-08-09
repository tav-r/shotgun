use std::error::Error;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::sync::Arc;
use url::Url;
use super::AnalyzeOptions;
use urlencoding;
use html_parser::{Dom,Node};

fn rand_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect()
}

fn rec_search_script_child(parent: &Node, needle: &str) -> bool {
    match parent {
        Node::Element(elt) => {
            if elt.name == "script" {
                for c in elt.children.iter() {
                    match c {
                        Node::Text(txt) => {
                            return txt.contains(needle);
                        }
                        _ => continue
                    }
                }
            } else {
                for c in elt.children.iter() {
                    if rec_search_script_child(c, needle) {
                        return true
                    }
                }
            }
        },
        _ => return false
    }

    false
}

fn reflected_in_script_block(body: &str, val: &str) -> Result<bool, Box<dyn Error>> {
    if let Ok(dom) = Dom::parse(&body) {
        for child in dom.children {
            if rec_search_script_child(&child, &val) {
                return Ok(true)
            }
        }                                    
    }

    Ok(false)
}

fn replace_vals(
    url: &mut Url,
) -> Vec<Url> {
    let mut res = Vec::new();
    for i in 0..url.query_pairs().count() {
        let mut new_url = url.clone();

        new_url.set_query(Some(
            &url.query_pairs().enumerate().map(
                |(j, p)| format!("{}={}", p.0, if i == j {rand_string()} else {String::from(p.1)})
            ).collect::<Vec<String>>().join("&")
        ));

        res.push(new_url);
    }

    res
}

pub async fn check_response(
    url: &mut Url,
    cookie_str: &str,
    options: &AnalyzeOptions
) -> Result<(), Box<dyn Error>> {
    for (i, url) in replace_vals(url).iter().enumerate() {
        let jar = Arc::new(reqwest::cookie::Jar::default());
        jar.add_cookie_str(cookie_str, &url.as_str().parse::<reqwest::Url>().unwrap());

        if let Ok(res) = reqwest::Client::builder()
            .cookie_provider(jar)
            .build()?
            .get(url.as_str())
            .send().await {
                let (key, val) = url.query_pairs().enumerate()
                    .filter(|(j, _)| *j == i)
                    .map(|(_, p)| (String::from(p.0), String::from(p.1)))
                    .collect::<Vec<(String, String)>>().pop().unwrap();

                let body = res.text().await?;

                if body.contains(&val) {
                    if options.picky { 
                        // make sure that at least one of the matches is not reflected in a URL,
                        // encoded URL or double-encoded URL
                        let n_val_reflected = body.matches(&val).count();
                        let n_url_reflected = body.matches(url.as_str()).count() +
                            body.matches(urlencoding::encode(url.as_str()).as_ref()).count() +
                            body.matches(urlencoding::encode(&urlencoding::encode(url.as_str())).as_ref()).count();

                        if n_url_reflected == n_val_reflected { continue }
                    }

                    if options.script_block {
                        if reflected_in_script_block(&body, &val)? {
                            println!("[{}] reflected {} in script block", url, key)
                        }
                    } else {
                        println!("[{}] reflected {}", url, key)
                    }
                }
            }
    }

    Ok(())
}
