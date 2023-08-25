use africastalking_rust::models::models::{MobileDataMessage, MobileDataRecipient};
use africastalking_rust::AfricasTalking;

pub async fn test_send_mobile_data_async(user_name: String, api_key: String, phone_number: String) {
    /*
    let user_name = USERNAME_SANDBOX.to_string();
    let api_key = API_KEY_SANDBOX.to_string();

    let user_name = USERNAME_PROD.to_string();
    let api_key = API_KEY_PROD.to_string();
    */

    let _result = AfricasTalking::new(user_name, api_key);

    if let Ok(africas_talking) = _result {
        //let phone_number = PHONE_NO.to_string();

        let _result = MobileDataRecipient::new(phone_number);
        if let Ok(mobile_data_recipient) = _result {
            let product_name = String::from("test");
            let mut _recipients: Vec<MobileDataRecipient> = Vec::new();
            _recipients.push(mobile_data_recipient);
            let _quantity = 50;
            let _unit = String::from("MB");
            let _validity = String::from("Day");
            let is_promo_bundle = false;

            let _result = MobileDataMessage::new(
                product_name,
                _recipients,
                _quantity,
                _unit,
                _validity,
                is_promo_bundle,
            );
            if let Ok(mobile_data_message) = _result {
                let _output = africas_talking.send_mobile_data_async(mobile_data_message);
                let _result = _output.await;
                if let Ok(result_message) = _result {
                    println!("result_message: {:?}", result_message);
                    if let Some(result_sms) = result_message {
                        println!("result_sms: {:?}", result_sms);
                    } else if let None = result_message {
                        println!("None");
                    } else {
                        println!("Unexpected error occured during processing");
                    }
                } else if let Err(e) = _result {
                    println!("{:?}", e);
                } else {
                    println!("Unexpected error occured during processing");
                }
            } else if let Err(e) = _result {
                println!("{:?}", e);
            } else {
                println!("Unexpected error occured during processing");
            }
        } else if let Err(e) = _result {
            println!("{:?}", e);
        } else {
            println!("Unexpected error occured during processing");
        }
    } else if let Err(e) = _result {
        println!("{:?}", e);
    } else {
        println!("Unexpected error occured during processing");
    }
}
