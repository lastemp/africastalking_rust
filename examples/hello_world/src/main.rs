use africastalking_rust::models::SmsMessage;
use africastalking_rust::AfricasTalking;

#[tokio::main]
async fn main() {
    let user_name = String::from("sandbox");
    let api_key = String::from("***");
    let _result = AfricasTalking::new(user_name, api_key);

    if let Ok(africas_talking) = _result {
        let _message = String::from("Have a good time!");
        let _to = String::from("+2547***");

        let _result = SmsMessage::new(_message, _to);
        if let Ok(sms_message) = _result {
            let _output = africas_talking.send_sms_message(sms_message);
            let _result = _output.await;
            if let Ok(result_sms_message) = _result {
                println!("result_sms_message: {:?}", result_sms_message);
                if let Some(result_sms) = result_sms_message {
                    println!("result_sms: {:?}", result_sms);
                } else if let None = result_sms_message {
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
