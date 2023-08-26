use africastalking_rust::models::models::{AirtimeMessage, AirtimeRecipient, FindAirtimeMessage};
use africastalking_rust::AfricasTalking;

pub async fn test_send_airtime_async(user_name: String, api_key: String, phone_number: String) {
    let _result = AfricasTalking::new(user_name, api_key);

    if let Ok(africas_talking) = _result {
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
                    if let Some(result_airtime) = result_message {
                        println!("result_airtime: {:?}", result_airtime);
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

pub async fn test_find_airtime_transaction_status_async(
    user_name: String,
    api_key: String,
    phone_number: String,
) {
    let _result = AfricasTalking::new(user_name, api_key);

    if let Ok(africas_talking) = _result {
        let transaction_id = String::from("18hga5hjam");

        let _result = FindAirtimeMessage::new(transaction_id);
        if let Ok(airtime_message) = _result {
            let _output = africas_talking.find_airtime_transaction_status_async(airtime_message);
            let _result = _output.await;
            if let Ok(result_message) = _result {
                println!("result_message: {:?}", result_message);
                if let Some(result_airtime) = result_message {
                    println!("result_airtime: {:?}", result_airtime);
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
