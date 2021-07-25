use crate::lib::ParamNameVal;
use std::error::Error;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::sync::Arc;

fn rand_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect()
}

fn replace_param_val_at(url: &str, par_name: &str, val: &str, new_val: &str, index: usize) -> String {
    format!(
        "{}{}",
        &url[..index],
        &url[index..].replacen(
            &format!("{}={}", par_name, val),
            &format!("{}={}", par_name, new_val),
            1
        ).trim()
    )
}

fn replace_vals<'a>(
    url: &str, param_vals: &'a Vec<ParamNameVal<'_>>,
    replace_val: &'a str
) -> Vec<(String, ParamNameVal<'a>)> {
    let mut res = Vec::new();
    for p in param_vals.iter() {
        res.push((
            replace_param_val_at(url, p.parameter, p.value, &replace_val, p.index),
            ParamNameVal{parameter: p.parameter, value: &replace_val, index: p.index}
        ))
    }

    res
}

pub async fn check_response(
    url: &str,
    params: &Vec<ParamNameVal<'_>>,
    cookie_str: &str,
    picky: bool
) -> Result<(), Box<dyn Error>> {
    for (url, new_param) in replace_vals(url, params, &rand_string()) {
        let jar = Arc::new(reqwest::cookie::Jar::default());
        jar.add_cookie_str(cookie_str, &url.parse::<reqwest::Url>().unwrap());

        if let Ok(res) = reqwest::Client::builder()
            .cookie_provider(jar)
            .build()?
            .get(&url)
            .send().await {
                let body = &res.text().await?;
                if body.contains(new_param.value) {
                    if picky { 
                        // make sure that at least one of the matches is not a 'key=value' match
                        let n_val_reflected = body.matches(new_param.value).count();
                        let n_url_reflected = body.matches(&format!("{}={}", new_param.parameter, new_param.value)).count() +
                            body.matches(&format!("{}%3d{}", new_param.parameter, new_param.value)).count() +
                            body.matches(&format!("{}%3D{}", new_param.parameter, new_param.value)).count();

                        if n_url_reflected == n_val_reflected { continue }
                    }

                    println!("[{}] reflected {}", url, new_param.parameter)
                }
            }
    }

    Ok(())
}
