use africastalking_rust::models::{
    AirtimeMessage, AirtimeRecipient, MobileDataMessage, MobileDataRecipient, SmsMessage,
};
use africastalking_rust::AfricasTalking;

// SANDBOX
const USERNAME_SANDBOX: &str = "***";
const API_KEY_SANDBOX: &str = "***";

// PROD
const USERNAME_PROD: &str = "***";
const API_KEY_PROD: &str = "***";

const PHONE_NO: &str = "***";

#[tokio::main]
async fn main() {
    //fn main() {
    //let x = test_send_message_async();
    //x.await;
    //test_send_message();
    //let x = test_send_airtime_async();
    let x = test_send_mobiledata_async();
    x.await;
}

async fn test_send_message_async() {
    /*
    let user_name = USERNAME_SANDBOX.to_string();
    let api_key = API_KEY_SANDBOX.to_string();
    */
    let user_name = USERNAME_PROD.to_string();
    let api_key = API_KEY_PROD.to_string();

    let _result = AfricasTalking::new(user_name, api_key);

    if let Ok(africas_talking) = _result {
        let _message = String::from("Have a wonderful time!");
        let _to = PHONE_NO.to_string();
        let _from = None;

        let _result = SmsMessage::new(_message, _to, _from);
        if let Ok(sms_message) = _result {
            let _output = africas_talking.send_message_async(sms_message);
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
}

fn test_send_message() {
    /*
    let user_name = USERNAME_SANDBOX.to_string();
    let api_key = API_KEY_SANDBOX.to_string();
    */
    let user_name = USERNAME_PROD.to_string();
    let api_key = API_KEY_PROD.to_string();

    let _result = AfricasTalking::new(user_name, api_key);

    if let Ok(africas_talking) = _result {
        let _message = String::from("Have a blessed time!");
        let _to = PHONE_NO.to_string();
        let _from = None;

        let _result = SmsMessage::new(_message, _to, _from);
        if let Ok(sms_message) = _result {
            let _result = africas_talking.send_message(sms_message);
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
}

async fn test_send_airtime_async() {
    let user_name = USERNAME_SANDBOX.to_string();
    let api_key = API_KEY_SANDBOX.to_string();
    /*
    let user_name = USERNAME_PROD.to_string();
    let api_key = API_KEY_PROD.to_string();
    */
    let _result = AfricasTalking::new(user_name, api_key);

    if let Ok(africas_talking) = _result {
        let phone_number = PHONE_NO.to_string();
        let _amount = 1;
        let currency_code = String::from("KES");

        let _result = AirtimeRecipient::new(phone_number, _amount, currency_code);
        if let Ok(airtime_recipient) = _result {
            let max_num_retry: u8 = 1;
            let mut _recipients: Vec<AirtimeRecipient> = Vec::new();
            _recipients.push(airtime_recipient);

            let _result = AirtimeMessage::new(max_num_retry, _recipients);
            if let Ok(airtime_message) = _result {
                let _output = africas_talking.send_airtime_async(airtime_message);
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

async fn test_send_mobiledata_async() {
    /*
    let user_name = USERNAME_SANDBOX.to_string();
    let api_key = API_KEY_SANDBOX.to_string();
    */
    let user_name = USERNAME_PROD.to_string();
    let api_key = API_KEY_PROD.to_string();

    let _result = AfricasTalking::new(user_name, api_key);

    if let Ok(africas_talking) = _result {
        let phone_number = PHONE_NO.to_string();

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
                let _output = africas_talking.send_mobiledata_async(mobile_data_message);
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
