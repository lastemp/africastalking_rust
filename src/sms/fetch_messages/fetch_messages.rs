use crate::models::models::ResultFetchSmsMessages;
use crate::util::util::build_headers;
use reqwest::StatusCode;

pub async fn fetch_sms_messages_async(
    last_received_id: u32,
    user_name: String,
    api_key: String,
    api_url: String,
) -> std::result::Result<ResultFetchSmsMessages, String> {
    let params = [
        ("username", user_name),
        ("lastReceivedId", last_received_id.to_string()),
    ];

    let client = reqwest::Client::new();
    let res = client
        .get(api_url)
        .headers(build_headers(api_key))
        .form(&params)
        .send()
        .await;

    match res {
        Err(e) => {
            return Err(e.to_string());
        }
        Ok(response) => match response.status() {
            StatusCode::CREATED => {
                match response.json::<ResultFetchSmsMessages>().await {
                    Ok(result_message) => {
                        // Handle success case
                        return Ok(result_message);
                    }
                    Err(_err) => {
                        // Handle error case
                        return Err(_err.to_string());
                    }
                }
            }
            s => {
                let mut _x = String::from("Request failed processing, status code: ");
                _x.push_str(&s.to_string());
                return Err(_x.to_string());
            }
        },
    };
}
