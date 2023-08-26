use crate::models::models::ResultSmsMessage;
use crate::util::util::build_headers;
use reqwest::StatusCode;

pub async fn send_message_async(
    _message: String,
    _to: String,
    _from: &Option<String>,
    _enqueue: &Option<bool>,
    user_name: String,
    api_key: String,
    api_url: String,
) -> std::result::Result<Option<ResultSmsMessage>, reqwest::Error> {
    /*
    let params = [
        ("username", user_name),
        ("to", _to),
        ("message", _message),
        ("from", _from),
    ];
    */

    let mut params = Vec::new();

    params.push(("username", user_name));
    params.push(("to", _to));
    params.push(("message", _message));

    if let Some(_from) = _from {
        params.push(("from", _from.to_string()));
    }

    if let Some(_enqueue) = _enqueue {
        // 1 to enable || 0 to disable
        let _enqueue: u8 = if *_enqueue { 1 } else { 0 };
        params.push(("enqueue", _enqueue.to_string()));
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
            return Err(e);
        }
        Ok(response) => match response.status() {
            StatusCode::CREATED => {
                let result_message = response.json::<ResultSmsMessage>().await?;

                return Ok(Some(result_message));
            }
            s => {
                return Ok(None);
            }
        },
    };
}

pub fn send_message(
    _message: String,
    _to: String,
    _from: &Option<String>,
    _enqueue: &Option<bool>,
    user_name: String,
    api_key: String,
    api_url: String,
) -> std::result::Result<Option<ResultSmsMessage>, reqwest::Error> {
    /*
    let params = [
        ("username", user_name),
        ("to", _to),
        ("message", _message),
        ("from", _from),
    ];
    */

    let mut params = Vec::new();

    params.push(("username", user_name));
    params.push(("to", _to));
    params.push(("message", _message));

    if let Some(_from) = _from {
        params.push(("from", _from.to_string()));
    }

    if let Some(_enqueue) = _enqueue {
        // 1 to enable || 0 to disable
        let _enqueue: u8 = if *_enqueue { 1 } else { 0 };
        params.push(("enqueue", _enqueue.to_string()));
    }

    let client = reqwest::blocking::Client::new();
    let res = client
        .post(api_url)
        .headers(build_headers(api_key))
        .form(&params)
        .send()?;

    match res.status() {
        StatusCode::CREATED => {
            let result_sms_message = res.json::<ResultSmsMessage>()?;

            return Ok(Some(result_sms_message));
        }
        s => {
            return Ok(None);
        }
    };
}
