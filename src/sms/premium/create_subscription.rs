use crate::models::models::ResultPremiumSmsSubscriptionMessage;
use crate::util::util::{build_headers, build_headers_non_api_key};
use reqwest::StatusCode;

pub async fn generate_checkout_token_async(
    phone_number: String,
    user_name: String,
    api_key: String,
    api_url: String,
) -> std::result::Result<Option<ResultPremiumSmsSubscriptionMessage>, reqwest::Error> {
    let params = [("phoneNumber", phone_number)];

    let client = reqwest::Client::new();
    let res = client
        .post(api_url)
        .headers(build_headers_non_api_key())
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
                    .json::<ResultPremiumSmsSubscriptionMessage>()
                    .await?;

                return Ok(Some(result_message));
            }
            s => {
                println!("status code: {:?}", s);
                return Ok(None);
            }
        },
    };
}

pub async fn subscribe_phone_number_async(
    short_code: String,
    _keyword: String,
    phone_number: String,
    checkout_token: String,
    user_name: String,
    api_key: String,
    api_url: String,
) -> std::result::Result<Option<ResultPremiumSmsSubscriptionMessage>, reqwest::Error> {
    let params = [
        ("username", user_name),
        ("shortCode", short_code),
        ("keyword", _keyword),
        ("phoneNumber", phone_number),
        ("checkoutToken", checkout_token),
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
                    .json::<ResultPremiumSmsSubscriptionMessage>()
                    .await?;

                return Ok(Some(result_message));
            }
            s => {
                println!("status code: {:?}", s);
                return Ok(None);
            }
        },
    };
}
