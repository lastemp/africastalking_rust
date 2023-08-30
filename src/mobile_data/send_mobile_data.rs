use crate::models::models::ResultMobileDataMessage;
use crate::util::util::build_headers;
use reqwest::StatusCode;

pub async fn send_mobile_data_async(
    product_name: String,
    _recipients: String,
    _quantity: u32,
    _unit: String,
    _validity: String,
    is_promo_bundle: bool,
    user_name: String,
    api_key: String,
    api_url: String,
) -> std::result::Result<ResultMobileDataMessage, String> {
    let params = [
        ("username", user_name),
        ("productName", product_name),
        ("recipients", _recipients),
        ("quantity", _quantity.to_string()),
        ("unit", _unit),
        ("validity", _validity),
        ("isPromoBundle", is_promo_bundle.to_string()),
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
            return Err(e.to_string());
        }
        Ok(response) => match response.status() {
            StatusCode::CREATED => {
                match response.json::<ResultMobileDataMessage>().await {
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
