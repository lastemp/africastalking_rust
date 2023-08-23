use crate::models::{ParamValue, ResultMobileDataMessage};
use crate::utils::build_headers;
use reqwest::StatusCode;

pub async fn send_mobiledata_async(
    product_name: String,
    _recipients: String,
    _quantity: u32,
    _unit: String,
    _validity: String,
    is_promo_bundle: bool,
    user_name: String,
    api_key: String,
    api_url: String,
) -> std::result::Result<Option<ResultMobileDataMessage>, reqwest::Error> {
    let params = [
        ("username", user_name),
        ("productName", product_name),
        ("recipients", _recipients),
        ("quantity", _quantity.to_string()),
        ("unit", _unit),
        ("validity", _validity),
        ("isPromoBundle", is_promo_bundle.to_string()),
    ];

    /*
    let params: Vec<(&str, ParamValue)> = vec![
        ("username", ParamValue::Str(user_name)),
        ("productName", ParamValue::Str(product_name)),
        ("recipients", ParamValue::Str(_recipients)),
        ("quantity", ParamValue::Int(_quantity)),
        ("unit", ParamValue::Str(_unit)),
        ("validity", ParamValue::Str(_validity)),
        ("isPromoBundle", ParamValue::Bool(is_promo_bundle)),
    ];

    let serialized = serde_urlencoded::to_string(&params).unwrap();

    println!("{}", serialized);

    println!("params: {:?}", &params);
    */

    /*
    let phone_number = String::from("***");
    let mobile_data_phone_number = MobileDataPhoneNumberRequest {
        phoneNumber: phone_number,
    };
    let product_name = String::from("test");
    let mut _recipients: Vec<MobileDataPhoneNumberRequest> = Vec::new();
    _recipients.push(mobile_data_phone_number);
    let _quantity = 50;
    let _unit = String::from("MB");
    let _validity = String::from("Day");
    let is_promo_bundle = false;
    let username: String = String::from("***");
    let mobile_data_request = MobileDataRequest {
        username: username,
        productName: product_name,
        recipients: _recipients,
        quantity: _quantity,
        unit: _unit,
        validity: _validity,
        isPromoBundle: is_promo_bundle,
    };

    println!("mobile_data_request: {:?}", &mobile_data_request);
    */

    let client = reqwest::Client::new();
    let res = client
        .post(api_url)
        .headers(build_headers(api_key))
        .form(&params)
        //.body(serialized)
        //.headers(build_headers_test(api_key))
        //.json(&mobile_data_request)
        .send()
        .await;

    match res {
        Err(e) => {
            return Err(e);
        }
        Ok(response) => match response.status() {
            StatusCode::CREATED => {
                let result_message = response.json::<ResultMobileDataMessage>().await?;

                return Ok(Some(result_message));
            }
            s => {
                /*
                let mut e = String::from("Received response status: ");
                let status_code = s.to_string();
                e.push_str(&status_code);
                return Err(e);
                */
                println!("Received response status: {:?}", s);
                return Ok(None);
            }
        },
    };
}