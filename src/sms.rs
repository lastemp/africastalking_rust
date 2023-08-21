use crate::models::ResultSmsMessage;
use crate::utils::build_headers;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use reqwest::StatusCode;

pub async fn send_sms_message(
    _message: String,
    _to: String,
    user_name: String,
    api_key: String,
    api_url: String,
) -> std::result::Result<Option<ResultSmsMessage>, reqwest::Error> {
    /*
    println!("_message: {:?}", &_message);
    println!("_to: {:?}", &_to);
    println!("user_name: {:?}", &user_name);
    println!("api_key: {:?}", &api_key);
    println!("api_url: {:?}", &api_url);
    */
    let params = [
        ("username", user_name),
        ("to", _to.to_string()),
        ("message", _message.to_string()),
    ];

    let client = reqwest::Client::new();
    let res = client
        .post(api_url)
        /*
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header(ACCEPT, "application/json")
        .header("apiKey", api_key)
        */
        .headers(build_headers(api_key))
        .form(&params)
        .send()
        //.await?; //The "?" after the await returns errors immediately and hence will not be captured on match clause below
        .await;

    match res {
        Err(e) => {
            return Err(e);
        }
        Ok(response) => match response.status() {
            StatusCode::CREATED => {
                let result_sms_message = response.json::<ResultSmsMessage>().await?;

                return Ok(Some(result_sms_message));
            }
            s => {
                println!("server response: {:?}", s.to_string());
                return Ok(None);
            }
        },
    };
}
