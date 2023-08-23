use crate::models::{AirtimeInputRecipient, MobileDataInputRecipient};
use reqwest::header::HeaderMap;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
//use serde_json::Result;

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
/*
pub fn build_headers_test(api_key: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("apiKey", api_key.parse().unwrap());

    headers
}
*/
pub fn parse_airtime_input_recipients(
    airtime_input_recipients: Vec<AirtimeInputRecipient>,
) -> String {
    //std::result::Result<String, serde_json::Error>
    let _result = serde_json::to_string(&airtime_input_recipients);
    let _result = match _result {
        Ok(x) => x,
        Err(_) => String::from(""),
    };
    _result
}

pub fn parse_mobile_data_input_recipients(
    mobile_data_input_recipients: Vec<MobileDataInputRecipient>,
) -> String {
    let _result = serde_json::to_string(&mobile_data_input_recipients);
    let _result = match _result {
        Ok(x) => x,
        Err(_) => String::from(""),
    };
    _result
}
