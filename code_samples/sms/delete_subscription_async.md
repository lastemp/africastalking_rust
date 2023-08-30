# delete subscription async

This example deletes premium sms subscriptions asynchronously.

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
    let x = sms::test_delete_subscription_async(user_name, api_key, phone_number);

    x.await;
}
```

## sms.rs

This module contains the function test_delete_subscription_async:

```rust
use africastalking_rust::AfricasTalking;
use africastalking_rust::models::models::DeleteSubscriptionMessage;

pub async fn test_delete_subscription_async(
    user_name: String,
    api_key: String,
    phone_number: String,
) {
    let _result = AfricasTalking::new(user_name, api_key);

    if let Ok(africas_talking) = _result {
        let short_code = String::from("1234");
        let _keyword = String::from("test");

        let _result = DeleteSubscriptionMessage::new(short_code, _keyword, phone_number);
        if let Ok(subscriptions_message) = _result {
            let _output = africas_talking.delete_subscription_async(subscriptions_message);
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
