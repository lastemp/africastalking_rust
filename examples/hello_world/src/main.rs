mod airtime;
mod mobile_data;
mod sms;
/*
use africastalking_rust::models::models::{
    AirtimeMessage, AirtimeRecipient, MobileDataMessage, MobileDataRecipient, SmsMessage,
};
use africastalking_rust::AfricasTalking;
*/

// SANDBOX
const USERNAME_SANDBOX: &str = "***";
const API_KEY_SANDBOX: &str = "***";

// PROD
const USERNAME_PROD: &str = "***";
const API_KEY_PROD: &str = "***";

const PHONE_NO: &str = "***";

#[tokio::main]
async fn main() {
    let user_name = USERNAME_SANDBOX.to_string();
    let api_key = API_KEY_SANDBOX.to_string();
    let phone_number = PHONE_NO.to_string();

    // sms
    //let x = sms::test_send_bulk_message_async(user_name, api_key, phone_number);
    //let x = sms::test_send_bulk_message(user_name, api_key, phone_number);
    //let x = sms::test_send_premium_message_async(user_name, api_key, phone_number);
    //let x = sms::test_fetch_sms_messages_async(user_name, api_key, phone_number);
    //let x = sms::test_create_sms_subscriptions_async(user_name, api_key, phone_number);
    //let x = sms::test_fetch_sms_subscriptions_async(user_name, api_key, phone_number);
    let x = sms::test_delete_subscription_async(user_name, api_key, phone_number);

    // airtime
    //let x = airtime::test_send_airtime_async(user_name, api_key, phone_number);

    // mobile_data
    //let x = mobile_data::test_send_mobile_data_async(user_name, api_key, phone_number);

    x.await;
}
