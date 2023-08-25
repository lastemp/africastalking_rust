use crate::models::models::ResultFetchSmsMessages;
use crate::util::util::build_headers;
use reqwest::StatusCode;

pub async fn fetch_sms_messages_async(
    last_received_id: u32,
    user_name: String,
    api_key: String,
    api_url: String,
) -> std::result::Result<Option<ResultFetchSmsMessages>, reqwest::Error> {
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
            return Err(e);
        }
        Ok(response) => match response.status() {
            StatusCode::CREATED => {
                let result_message = response.json::<ResultFetchSmsMessages>().await?;

                return Ok(Some(result_message));
            }
            s => {
                return Ok(None);
            }
        },
    };
}
