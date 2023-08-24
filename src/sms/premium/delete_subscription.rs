use crate::models::ResultPremiumSmsDeleteSubscriptionMessage;
use crate::utils::build_headers;
use reqwest::StatusCode;

pub async fn delete_subscription_async(
    short_code: String,
    _keyword: String,
    phone_number: String,
    user_name: String,
    api_key: String,
    api_url: String,
) -> std::result::Result<Option<ResultPremiumSmsDeleteSubscriptionMessage>, reqwest::Error> {
    let params = [
        ("username", user_name),
        ("shortCode", short_code),
        ("keyword", _keyword),
        ("phoneNumber", phone_number),
    ];

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
                let result_message = response
                    .json::<ResultPremiumSmsDeleteSubscriptionMessage>()
                    .await?;

                return Ok(Some(result_message));
            }
            s => {
                return Ok(None);
            }
        },
    };
}
