use crate::models::models::ResultSmsMessage;
use crate::util::util::build_headers;
use reqwest::StatusCode;

pub async fn send_message_async(
    _message: String,
    _to: String,
    _from: String,
    _keyword: &Option<String>,
    _enqueue: &Option<bool>,
    link_id: &Option<String>,
    retry_duration_in_hours: &Option<u8>,
    request_id: &Option<String>,
    user_name: String,
    api_key: String,
    api_url: String,
) -> std::result::Result<ResultSmsMessage, String> {
    let mut params = Vec::new();

    params.push(("username", user_name));
    params.push(("to", _to));
    params.push(("message", _message));
    params.push(("from", _from));

    if let Some(_keyword) = _keyword {
        params.push(("keyword", _keyword.to_string()));
    }

    if let Some(_enqueue) = _enqueue {
        // 1 to enable || 0 to disable
        let _enqueue: u8 = if *_enqueue { 1 } else { 0 };
        params.push(("enqueue", _enqueue.to_string()));
    }

    if let Some(link_id) = link_id {
        params.push(("linkId", link_id.to_string()));
    }

    if let Some(retry_duration_in_hours) = retry_duration_in_hours {
        params.push(("retryDurationInHours", retry_duration_in_hours.to_string()));
    }

    if let Some(request_id) = request_id {
        params.push(("requestId", request_id.to_string()));
    }

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
                match response.json::<ResultSmsMessage>().await {
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
