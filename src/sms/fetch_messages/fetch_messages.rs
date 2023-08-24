use crate::models::ResultFetchSmsMessages;
use crate::utils::build_headers;
use reqwest::StatusCode;

pub async fn fetch_sms_messages_async(
    last_received_id: String,
    user_name: String,
    api_key: String,
    api_url: String,
) -> std::result::Result<Option<ResultFetchSmsMessages>, reqwest::Error> {
    let params = [
        ("username", user_name),
        ("lastReceivedId", last_received_id),
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
