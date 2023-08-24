use crate::models::ResultPremiumSmsFetchSubscriptionsMessage;
use crate::utils::build_headers;
use reqwest::StatusCode;

pub async fn fetch_sms_subscriptions_async(
    short_code: String,
    _keyword: String,
    last_received_id: String,
    user_name: String,
    api_key: String,
    api_url: String,
) -> std::result::Result<Option<ResultPremiumSmsFetchSubscriptionsMessage>, reqwest::Error> {
    let params = [
        ("username", user_name),
        ("shortCode", short_code),
        ("keyword", _keyword),
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
                let result_message = response
                    .json::<ResultPremiumSmsFetchSubscriptionsMessage>()
                    .await?;

                return Ok(Some(result_message));
            }
            s => {
                return Ok(None);
            }
        },
    };
}