mod sms {
    pub mod bulk {
        pub mod send_sms;
    }
    pub mod premium {
        pub mod create_subscription;
        pub mod delete_subscription;
        pub mod fetch_subscriptions;
        pub mod send_sms;
    }
    pub mod fetch_messages {
        pub mod fetch_messages;
    }
}

mod airtime {
    pub mod find_transaction_status;
    pub mod send_airtime;
}

mod mobile_data {
    pub mod find_transaction;
    pub mod send_mobile_data;
}

mod util {
    pub mod util;
}

pub mod models {
    pub mod models;
}

use models::models::{
    AirtimeInputRecipient, AirtimeMessage, BulkSmsMessage, CreateSubscriptionsMessage,
    DeleteSubscriptionMessage, FetchSmsMessage, FetchSubscriptionsMessage, FindAirtimeMessage,
    FindMobileDataMessage, MobileDataMessage, PremiumSmsMessage, ResultAirtimeMessage,
    ResultFetchSmsMessages, ResultFetchTransactionAirtimeMessage,
    ResultFetchTransactionMobileDataMessage, ResultMobileDataMessage,
    ResultPremiumSmsDeleteSubscriptionMessage, ResultPremiumSmsFetchSubscriptionsMessage,
    ResultPremiumSmsSubscriptionMessage, ResultSmsMessage,
};
use util::util::parse_airtime_input_recipients;

use crate::{
    models::models::MobileDataInputRecipient, util::util::parse_mobile_data_input_recipients,
};

const SMS_URL_SANDBOX: &str = "https://api.sandbox.africastalking.com/version1/messaging";
const SMS_URL_PROD: &str = "https://api.africastalking.com/version1/messaging";
const CREATE_SUBSCRIPTION_TOKEN_SMS_URL_SANDBOX: &str =
    "https://api.sandbox.africastalking.com/checkout/token/create";
const CREATE_SUBSCRIPTION_TOKEN_SMS_URL_PROD: &str =
    "https://api.africastalking.com/checkout/token/create";
const CREATE_SUBSCRIPTION_SMS_URL_SANDBOX: &str =
    "https://api.sandbox.africastalking.com/version1/subscription/create";
const CREATE_SUBSCRIPTION_SMS_URL_PROD: &str =
    "https://content.africastalking.com/version1/subscription/create";
const FETCH_SUBSCRIPTION_SMS_URL_SANDBOX: &str =
    "https://api.sandbox.africastalking.com/version1/subscription";
const FETCH_SUBSCRIPTION_SMS_URL_PROD: &str =
    "https://content.africastalking.com/version1/subscription";
const DELETE_SUBSCRIPTION_SMS_URL_SANDBOX: &str =
    "https://api.sandbox.africastalking.com/version1/subscription/delete";
const DELETE_SUBSCRIPTION_SMS_URL_PROD: &str =
    "https://content.africastalking.com/version1/subscription/delete";
const DEFAULT_SENDER: &str = "AFRICASTKNG";
const AIRTIME_URL_SANDBOX: &str = "https://api.sandbox.africastalking.com/version1/airtime/send";
const AIRTIME_URL_PROD: &str = "https://api.africastalking.com/version1/airtime/send";
const MOBILE_DATA_URL_SANDBOX: &str =
    "https://payments.sandbox.africastalking.com/mobile/data/request";
const MOBILE_DATA_URL_PROD: &str = "https://payments.africastalking.com/mobile/data/request";

#[derive(Debug)]
pub struct AfricasTalking {
    user_name: String,
    api_key: String,
    sms_url: String,
    create_subscription_token_sms_url: String,
    create_subscription_sms_url: String,
    fetch_subscription_sms_url: String,
    delete_subscription_sms_url: String,
    airtime_url: String,
    mobile_data_url: String,
    _env: String,
}

impl AfricasTalking {
    pub fn new(user_name: String, api_key: String) -> Result<Self, String> {
        if user_name.is_empty() || user_name.replace(" ", "").trim().len() == 0 {
            return Err(String::from("user name is empty"));
        }

        if api_key.is_empty() || api_key.replace(" ", "").trim().len() == 0 {
            return Err(String::from("api key is empty"));
        }

        let _env = if user_name.eq_ignore_ascii_case(&String::from("sandbox")) {
            "sandbox"
        } else {
            "prod"
        };

        let sms_url: String = match _env {
            "prod" => SMS_URL_PROD.to_string(),
            _ => SMS_URL_SANDBOX.to_string(),
        };

        let create_subscription_token_sms_url: String = match _env {
            "prod" => CREATE_SUBSCRIPTION_TOKEN_SMS_URL_PROD.to_string(),
            _ => CREATE_SUBSCRIPTION_TOKEN_SMS_URL_SANDBOX.to_string(),
        };

        let create_subscription_sms_url: String = match _env {
            "prod" => CREATE_SUBSCRIPTION_SMS_URL_PROD.to_string(),
            _ => CREATE_SUBSCRIPTION_SMS_URL_SANDBOX.to_string(),
        };

        let fetch_subscription_sms_url: String = match _env {
            "prod" => FETCH_SUBSCRIPTION_SMS_URL_PROD.to_string(),
            _ => FETCH_SUBSCRIPTION_SMS_URL_SANDBOX.to_string(),
        };

        let delete_subscription_sms_url: String = match _env {
            "prod" => DELETE_SUBSCRIPTION_SMS_URL_PROD.to_string(),
            _ => DELETE_SUBSCRIPTION_SMS_URL_SANDBOX.to_string(),
        };

        let airtime_url: String = match _env {
            "prod" => AIRTIME_URL_PROD.to_string(),
            _ => AIRTIME_URL_SANDBOX.to_string(),
        };

        let mobile_data_url: String = match _env {
            "prod" => MOBILE_DATA_URL_PROD.to_string(),
            _ => MOBILE_DATA_URL_SANDBOX.to_string(),
        };

        let _env = _env.to_string();

        Ok(Self {
            user_name,
            api_key,
            sms_url,
            create_subscription_token_sms_url,
            create_subscription_sms_url,
            fetch_subscription_sms_url,
            delete_subscription_sms_url,
            airtime_url,
            mobile_data_url,
            _env,
        })
    }

    // SMS
    pub async fn send_bulk_message_async(
        &self,
        sms_message: BulkSmsMessage,
    ) -> std::result::Result<Option<ResultSmsMessage>, reqwest::Error> {
        let _message = sms_message.get_message();
        let _to: String = sms_message.get_recipient();
        let _from = sms_message.get_sender();
        let _enqueue = sms_message.get_enqueue();
        let user_name = &self.user_name;
        let api_key = &self.api_key;
        let api_url = &self.sms_url;
        /*/
        let _from = match _from {
            Some(_x) => _x.to_string(),
            _ => DEFAULT_SENDER.to_string(),
        };
        */

        let _output = sms::bulk::send_sms::send_message_async(
            _message,
            _to,
            _from,
            _enqueue,
            user_name.to_string(),
            api_key.to_string(),
            api_url.to_string(),
        );

        let _result = _output.await;

        _result
    }

    // SMS
    pub fn send_bulk_message(
        &self,
        sms_message: BulkSmsMessage,
    ) -> std::result::Result<Option<ResultSmsMessage>, reqwest::Error> {
        let _message = sms_message.get_message();
        let _to: String = sms_message.get_recipient();
        let _from = sms_message.get_sender();
        let _enqueue = sms_message.get_enqueue();
        let user_name = &self.user_name;
        let api_key = &self.api_key;
        let api_url = &self.sms_url;
        /*
        let _from = match _from {
            Some(_x) => _x.to_string(),
            _ => DEFAULT_SENDER.to_string(),
        };
        */
        let _result = sms::bulk::send_sms::send_message(
            _message,
            _to,
            _from,
            _enqueue,
            user_name.to_string(),
            api_key.to_string(),
            api_url.to_string(),
        );

        _result
    }

    // Premium SMS
    pub async fn send_premium_message_async(
        &self,
        sms_message: PremiumSmsMessage,
    ) -> std::result::Result<Option<ResultSmsMessage>, reqwest::Error> {
        let _message = sms_message.get_message();
        let _to: String = sms_message.get_recipient();
        let _from = sms_message.get_sender();
        let _keyword = sms_message.get_keyword();
        let _enqueue = sms_message.get_enqueue();
        let link_id = sms_message.get_link_id();
        let retry_duration_in_hours = sms_message.get_retry_duration_in_hours();
        let request_id = sms_message.get_request_id();
        let user_name = &self.user_name;
        let api_key = &self.api_key;
        let api_url = &self.sms_url;

        let _output = sms::premium::send_sms::send_message_async(
            _message,
            _to,
            _from,
            _keyword,
            _enqueue,
            link_id,
            retry_duration_in_hours,
            request_id,
            user_name.to_string(),
            api_key.to_string(),
            api_url.to_string(),
        );

        let _result = _output.await;

        _result
    }

    // SMS
    pub async fn fetch_sms_messages_async(
        &self,
        fetch_sms_message: FetchSmsMessage,
    ) -> std::result::Result<Option<ResultFetchSmsMessages>, reqwest::Error> {
        let last_received_id = fetch_sms_message.get_last_received_id();
        let user_name = &self.user_name;
        let api_key = &self.api_key;
        let api_url = &self.sms_url;

        let last_received_id = match last_received_id {
            Some(_x) => _x,
            _ => 0,
        };

        let _output = sms::fetch_messages::fetch_messages::fetch_sms_messages_async(
            last_received_id,
            user_name.to_string(),
            api_key.to_string(),
            api_url.to_string(),
        );

        let _result = _output.await;

        _result
    }

    // Premium SMS
    pub async fn create_sms_subscriptions_async(
        &self,
        subscriptions_message: CreateSubscriptionsMessage,
    ) -> std::result::Result<Option<ResultPremiumSmsSubscriptionMessage>, reqwest::Error> {
        let short_code = subscriptions_message.get_short_code();
        let _keyword = subscriptions_message.get_keyword();
        let phone_number = subscriptions_message.get_phone_number();
        let user_name = &self.user_name;
        let api_key = &self.api_key;
        let api_url = &self.create_subscription_token_sms_url;

        let _output = sms::premium::create_subscription::generate_checkout_token_async(
            phone_number.to_string(),
            user_name.to_string(),
            api_key.to_string(),
            api_url.to_string(),
        );

        let _result = _output.await;

        let k = String::from(""); //Default value.

        let (checkout_token, _description) = match _result {
            Ok(_output) => {
                let _x = if let Some(_x) = _output {
                    let _token = _x.token.as_ref().unwrap_or(&k);
                    let _description = _x.description.as_ref().unwrap_or(&k);
                    (_token.to_string(), _description.to_string())
                } else {
                    (k, String::from(""))
                };
                _x
            }
            _ => (k, String::from("")),
        };

        println!("checkout_token: {:?}", &checkout_token);
        println!("_description: {:?}", &_description);

        // Proceed with processing if _description is Success
        let is_successful = if _description.eq_ignore_ascii_case(&String::from("success")) {
            true
        } else {
            false
        };

        let api_url = &self.create_subscription_sms_url;
        let _output = sms::premium::create_subscription::subscribe_phone_number_async(
            short_code.to_string(),
            _keyword.to_string(),
            phone_number.to_string(),
            checkout_token,
            user_name.to_string(),
            api_key.to_string(),
            api_url.to_string(),
        );

        let _result = _output.await;

        _result
    }

    // Premium SMS
    pub async fn fetch_sms_subscriptions_async(
        &self,
        fetch_subscriptions_message: FetchSubscriptionsMessage,
    ) -> std::result::Result<Option<ResultPremiumSmsFetchSubscriptionsMessage>, reqwest::Error>
    {
        let short_code = fetch_subscriptions_message.get_short_code();
        let _keyword = fetch_subscriptions_message.get_keyword();
        let last_received_id = fetch_subscriptions_message.get_last_received_id();
        let user_name = &self.user_name;
        let api_key = &self.api_key;
        let api_url = &self.fetch_subscription_sms_url;

        let last_received_id = match last_received_id {
            Some(_x) => _x,
            _ => 0,
        };

        let _output = sms::premium::fetch_subscriptions::fetch_sms_subscriptions_async(
            short_code.to_string(),
            _keyword.to_string(),
            last_received_id,
            user_name.to_string(),
            api_key.to_string(),
            api_url.to_string(),
        );

        let _result = _output.await;

        _result
    }

    // Premium SMS
    pub async fn delete_subscription_async(
        &self,
        delete_subscription_message: DeleteSubscriptionMessage,
    ) -> std::result::Result<Option<ResultPremiumSmsDeleteSubscriptionMessage>, reqwest::Error>
    {
        let short_code = delete_subscription_message.get_short_code();
        let _keyword = delete_subscription_message.get_keyword();
        let phone_number = delete_subscription_message.get_phone_number();
        let user_name = &self.user_name;
        let api_key = &self.api_key;
        let api_url = &self.delete_subscription_sms_url;

        let _output = sms::premium::delete_subscription::delete_subscription_async(
            short_code.to_string(),
            _keyword.to_string(),
            phone_number.to_string(),
            user_name.to_string(),
            api_key.to_string(),
            api_url.to_string(),
        );

        let _result = _output.await;

        _result
    }

    // Airtime
    pub async fn send_airtime_async(
        &self,
        airtime_message: AirtimeMessage,
    ) -> std::result::Result<Option<ResultAirtimeMessage>, reqwest::Error> {
        let max_num_retry = airtime_message.get_max_num_retry();
        let _recipients = airtime_message.get_recipients();
        let user_name = &self.user_name;
        let api_key = &self.api_key;
        let api_url = &self.airtime_url;
        let mut airtime_input_recipients: Vec<AirtimeInputRecipient> = Vec::new();
        let _x = _recipients.len();
        if _x > 0 {
            for _recipient in _recipients.iter() {
                let phone_number: String = _recipient.get_phone_number();
                let _amount: u32 = _recipient.get_amount();
                let currency_code: String = _recipient.get_currency_code();
                let mut my_amount: String = currency_code;
                let _k = String::from(" ");
                my_amount.push_str(&_k);
                my_amount.push_str(&_amount.to_string());

                let airtime_input_recipient = AirtimeInputRecipient {
                    phoneNumber: phone_number,
                    amount: my_amount,
                };

                airtime_input_recipients.push(airtime_input_recipient);
            }
        }

        // convert _recipients to json
        let _recipients = parse_airtime_input_recipients(airtime_input_recipients);
        println!("_recipients: {:?}", &_recipients);

        let _output = airtime::send_airtime::send_airtime_async(
            max_num_retry,
            _recipients,
            user_name.to_string(),
            api_key.to_string(),
            api_url.to_string(),
        );

        let _result = _output.await;

        _result
    }

    // Airtime
    pub async fn find_airtime_transaction_status_async(
        &self,
        find_airtime_message: FindAirtimeMessage,
    ) -> std::result::Result<Option<ResultFetchTransactionAirtimeMessage>, reqwest::Error> {
        let transaction_id = find_airtime_message.get_transaction_id();
        let user_name = &self.user_name;
        let api_key = &self.api_key;
        let api_url = &self.sms_url;

        let _output = airtime::find_transaction_status::find_airtime_transaction_status_async(
            transaction_id.to_string(),
            user_name.to_string(),
            api_key.to_string(),
            api_url.to_string(),
        );

        let _result = _output.await;

        _result
    }

    // Mobile Data
    pub async fn send_mobile_data_async(
        &self,
        mobile_data_message: MobileDataMessage,
    ) -> std::result::Result<Option<ResultMobileDataMessage>, reqwest::Error> {
        let product_name = mobile_data_message.get_product_name();
        let _recipients = mobile_data_message.get_recipients();
        let _quantity = mobile_data_message.get_quantity();
        let _unit = mobile_data_message.get_unit();
        let _validity = mobile_data_message.get_validity();
        let is_promo_bundle = mobile_data_message.get_is_promo_bundle();
        let user_name = &self.user_name;
        let api_key = &self.api_key;
        let api_url = &self.mobile_data_url;
        let mut mobile_data_input_recipients: Vec<MobileDataInputRecipient> = Vec::new();
        let _x = _recipients.len();
        if _x > 0 {
            for _recipient in _recipients.iter() {
                let phone_number: String = _recipient.get_phone_number();

                let mobile_data_input_recipient = MobileDataInputRecipient {
                    phoneNumber: phone_number,
                };

                mobile_data_input_recipients.push(mobile_data_input_recipient);
            }
        }

        // convert _recipients to json
        let _recipients = parse_mobile_data_input_recipients(mobile_data_input_recipients);
        println!("_recipients: {:?}", &_recipients);

        let _output = mobile_data::send_mobile_data::send_mobile_data_async(
            product_name,
            _recipients,
            _quantity,
            _unit,
            _validity,
            is_promo_bundle,
            user_name.to_string(),
            api_key.to_string(),
            api_url.to_string(),
        );

        let _result = _output.await;

        _result
    }

    // Mobile Data
    pub async fn find_mobile_data_transaction_async(
        &self,
        find_mobile_data_message: FindMobileDataMessage,
    ) -> std::result::Result<Option<ResultFetchTransactionMobileDataMessage>, reqwest::Error> {
        let transaction_id = find_mobile_data_message.get_transaction_id();
        let user_name = &self.user_name;
        let api_key = &self.api_key;
        let api_url = &self.sms_url;

        let _output = mobile_data::find_transaction::find_mobile_data_transaction_async(
            transaction_id.to_string(),
            user_name.to_string(),
            api_key.to_string(),
            api_url.to_string(),
        );

        let _result = _output.await;

        _result
    }
}

/*
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
