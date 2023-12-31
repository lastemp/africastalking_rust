use crate::models::models::ResultAirtimeMessage;
use crate::util::util::build_headers;
use reqwest::StatusCode;

pub async fn send_airtime_async(
    max_num_retry: Option<u8>,
    _recipients: String,
    user_name: String,
    api_key: String,
    api_url: String,
) -> std::result::Result<ResultAirtimeMessage, String> {
    let mut params = Vec::new();

    params.push(("username", user_name));

    if let Some(max_num_retry) = max_num_retry {
        params.push(("maxNumRetry", max_num_retry.to_string()));
    }

    params.push(("recipients", _recipients));

    let client = reqwest::Client::new();
    let res = client
        .post(api_url)
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
                match response.json::<ResultAirtimeMessage>().await {
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
