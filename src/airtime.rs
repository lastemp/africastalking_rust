use crate::models::ResultAirtimeMessage;
use crate::utils::build_headers;
use reqwest::StatusCode;

pub async fn send_airtime_async(
    max_num_retry: u8,
    _recipients: String,
    user_name: String,
    api_key: String,
    api_url: String,
) -> std::result::Result<Option<ResultAirtimeMessage>, reqwest::Error> {
    let params = [
        ("username", user_name),
        ("maxNumRetry", max_num_retry.to_string()),
        ("recipients", _recipients),
    ];
    println!("params: {:?}", &params);
    let client = reqwest::Client::new();
    let res = client
        .post(api_url)
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
                let result_message = response.json::<ResultAirtimeMessage>().await?;

                return Ok(Some(result_message));
            }
            s => {
                return Ok(None);
            }
        },
    };
}
