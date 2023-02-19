use lazy_static::lazy_static;
use regex::Regex;

fn extract_host(url: String) -> String {
    const URL_REGEX: &str = r#"^http[s]?://(?P<host>[^:/\s]+)(((/\w+)*/))?(.*)$"#;

    lazy_static! {
        static ref RE: Regex = Regex::new(URL_REGEX).unwrap();
    }
    let optional_groups = RE.captures(&url);
    match optional_groups {
        None => url,  // Simply return the url as it is if the regex didn't match
        Some(groups) => groups.name("host").unwrap().as_str().to_string(),
    }
}

pub fn sanitize_host(host: &str) -> String {
    let host_without_whitespace: String = host.chars().filter(|c| !c.is_whitespace()).collect();
    if host_without_whitespace.starts_with("http") {
        extract_host(host_without_whitespace)
    } else {
        host_without_whitespace
    }
}
