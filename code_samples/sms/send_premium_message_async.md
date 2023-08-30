# send premium message async

This example sends premium sms message asynchronously.

## main.rs

This should contain below code:

```rust
mod sms;

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

    // sms
    let x = sms::test_send_premium_message_async(user_name, api_key, phone_number);

    x.await;
}
```

## sms.rs

This module contains the function test_send_premium_message_async:

```rust
use africastalking_rust::AfricasTalking;
use africastalking_rust::models::models::PremiumSmsMessage;

pub async fn test_send_premium_message_async(
    user_name: String,
    api_key: String,
    phone_number: String,
) {
    let _result = AfricasTalking::new(user_name, api_key);

    if let Ok(africas_talking) = _result {
        let _message = String::from("Have a wonderful time!");
        let _to = phone_number;
        let _from = String::from("***");
        let _keyword = None; // optional parameter
        let _enqueue = None; // optional parameter
        let link_id = None; // optional parameter
        let retry_duration_in_hours = None; // optional parameter
        let request_id = None; // optional parameter

        let _result = PremiumSmsMessage::new(
            _message,
            _to,
            _from,
            _keyword,
            _enqueue,
            link_id,
            retry_duration_in_hours,
            request_id,
        );
        if let Ok(sms_message) = _result {
            let _output = africas_talking.send_premium_message_async(sms_message);
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
}
```
