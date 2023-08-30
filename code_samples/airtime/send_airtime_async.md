# send airtime async

This example sends airtime message asynchronously.

## main.rs

This should contain below code:

```rust
mod airtime;

// SANDBOX
const USERNAME_SANDBOX: &str = "***";
const API_KEY_SANDBOX: &str = "***";

// PROD
const USERNAME_PROD: &str = "***";
const API_KEY_PROD: &str = "***";

const PHONE_NO: &str = "+2547***";

#[tokio::main]
async fn main() {
    let user_name = USERNAME_SANDBOX.to_string();
    let api_key = API_KEY_SANDBOX.to_string();
    let phone_number = PHONE_NO.to_string();

    // airtime
    let x = airtime::test_send_airtime_async(user_name, api_key, phone_number);

    x.await;
}
```

## airtime.rs

This module contains the function test_send_airtime_async:

```rust
use africastalking_rust::models::models::{AirtimeMessage, AirtimeRecipient};
use africastalking_rust::AfricasTalking;

pub async fn test_send_airtime_async(user_name: String, api_key: String, phone_number: String) {
    let _result = AfricasTalking::new(user_name, api_key);

    if let Ok(africas_talking) = _result {
        let _amount = 100;
        let currency_code = String::from("KES");

        let _result = AirtimeRecipient::new(phone_number, _amount, currency_code);
        if let Ok(airtime_recipient) = _result {
            let max_num_retry = None; // optional parameter
            let mut _recipients: Vec<AirtimeRecipient> = Vec::new();
            _recipients.push(airtime_recipient);

            let _result = AirtimeMessage::new(max_num_retry, _recipients);
            if let Ok(airtime_message) = _result {
                let _output = africas_talking.send_airtime_async(airtime_message);
                let _result = _output.await;
                if let Ok(result_message) = _result {
                    println!("result_message: {:?}", result_message);
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
```
