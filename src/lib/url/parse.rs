use crate::lib::ParamNameVal;

pub fn get_params<'a>(url: &'a str) -> Vec<ParamNameVal> {
    let mut res = Vec::new();

    if let Some(mut start) = url.find("?") {
        start += 1;  // skip question mark

        loop {
            let pos = url.get(start..)
                .unwrap_or("")
                .find("&")
                .unwrap_or(url.len() - start);
            let part = &url[start..start + pos];

            if let Some(index) = part.find("=") {
                res.push(ParamNameVal{
                    parameter: &part[..index],
                    value: &part[index + 1..],
                    index: start
                });
            }

            start += 1 /* skip ampersand */ + pos;

            if start > url.len() {
                break
            }
        }
    }

    res
}
