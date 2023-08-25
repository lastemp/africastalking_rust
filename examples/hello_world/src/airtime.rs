use africastalking_rust::models::models::{AirtimeMessage, AirtimeRecipient};
use africastalking_rust::AfricasTalking;

pub async fn test_send_airtime_async(user_name: String, api_key: String, phone_number: String) {
    /*
    let user_name = USERNAME_SANDBOX.to_string();
    let api_key = API_KEY_SANDBOX.to_string();

    let user_name = USERNAME_PROD.to_string();
    let api_key = API_KEY_PROD.to_string();
    */
    let _result = AfricasTalking::new(user_name, api_key);

    if let Ok(africas_talking) = _result {
        //let phone_number = PHONE_NO.to_string();
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
