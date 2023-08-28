use crate::models::models::ResultPremiumSmsFetchSubscriptionsMessage;
use crate::util::util::build_headers;
use reqwest::StatusCode;

pub async fn fetch_sms_subscriptions_async(
    short_code: String,
    _keyword: String,
    last_received_id: u32,
    user_name: String,
    api_key: String,
    api_url: String,
) -> std::result::Result<ResultPremiumSmsFetchSubscriptionsMessage, String> {
    let params = [
        ("username", user_name),
        ("shortCode", short_code),
        ("keyword", _keyword),
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
                /*
                let result_message = response
                    .json::<ResultPremiumSmsFetchSubscriptionsMessage>()
                    .await?;

                return Ok(Some(result_message));
                */
                match response
                    .json::<ResultPremiumSmsFetchSubscriptionsMessage>()
                    .await
                {
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
                //println!("status code: {:?}", s);
                //return Ok(None);
                let mut _x = String::from("Request failed processing, status code: ");
                _x.push_str(&s.to_string());
                return Err(_x.to_string());
            }
        },
    };
}
