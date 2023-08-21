use reqwest::header::HeaderMap;
use reqwest::header::{ACCEPT, CONTENT_TYPE};

pub fn build_headers(api_key: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("apiKey", api_key.parse().unwrap());

    headers
}
