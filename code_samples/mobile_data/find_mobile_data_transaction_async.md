# find mobile data transaction async

This example finds mobile data transaction asynchronously.

## main.rs

This should contain below code:

```rust
mod mobile_data;

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

    // mobile_data
    let x = mobile_data::test_find_mobile_data_transaction_async(user_name, api_key, phone_number);

    x.await;
}
```

## mobile_data.rs

This module contains the function test_find_mobile_data_transaction_async:

```rust
use africastalking_rust::AfricasTalking;
use africastalking_rust::models::models::FindMobileDataMessage;

pub async fn test_find_mobile_data_transaction_async(
    user_name: String,
    api_key: String,
    phone_number: String,
) {
    let _result = AfricasTalking::new(user_name, api_key);

    if let Ok(africas_talking) = _result {
        let transaction_id = String::from("***");

        let _result = FindMobileDataMessage::new(transaction_id);
        if let Ok(mobile_data_message) = _result {
            let _output = africas_talking.find_mobile_data_transaction_async(mobile_data_message);
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