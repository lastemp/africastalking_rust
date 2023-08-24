mod sms {
    pub mod bulk {
        pub mod send_sms;
    }
}

mod airtime {
    pub mod send_airtime;
}

mod mobile_data {
    pub mod send_mobile_data;
}

mod util {
    pub mod util;
}

pub mod models {
    pub mod models;
}

use models::models::{
    AirtimeInputRecipient, AirtimeMessage, MobileDataMessage, ResultAirtimeMessage,
    ResultMobileDataMessage, ResultSmsMessage, SmsMessage,
};
use util::util::parse_airtime_input_recipients;

use crate::{
    models::models::MobileDataInputRecipient, util::util::parse_mobile_data_input_recipients,
};

const SMS_URL_SANDBOX: &str = "https://api.sandbox.africastalking.com/version1/messaging";
const SMS_URL_PROD: &str = "https://api.africastalking.com/version1/messaging";
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

        let _env = if user_name
            .to_lowercase()
            .eq_ignore_ascii_case(&String::from("sandbox"))
        {
            "sandbox"
        } else {
            "prod"
        };

        let sms_url: String = match _env {
            "prod" => SMS_URL_PROD.to_string(),
            _ => SMS_URL_SANDBOX.to_string(),
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
            airtime_url,
            mobile_data_url,
            _env,
        })
    }

    // SMS
    pub async fn send_bulk_message_async(
        &self,
        sms_message: SmsMessage,
    ) -> std::result::Result<Option<ResultSmsMessage>, reqwest::Error> {
        let _message = sms_message.get_message();
        let _to: String = sms_message.get_recipient();
        let _from = sms_message.get_sender();
        let user_name = &self.user_name;
        let api_key = &self.api_key;
        let api_url = &self.sms_url;

        let _from = match _from {
            Some(_x) => _x.to_string(),
            _ => DEFAULT_SENDER.to_string(),
        };

        let _output = sms::bulk::send_sms::send_message_async(
            _message,
            _to,
            _from,
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
        sms_message: SmsMessage,
    ) -> std::result::Result<Option<ResultSmsMessage>, reqwest::Error> {
        let _message = sms_message.get_message();
        let _to: String = sms_message.get_recipient();
        let _from = sms_message.get_sender();
        let user_name = &self.user_name;
        let api_key = &self.api_key;
        let api_url = &self.sms_url;

        let _from = match _from {
            Some(_x) => _x.to_string(),
            _ => DEFAULT_SENDER.to_string(),
        };

        let _result = sms::bulk::send_sms::send_message(
            _message,
            _to,
            _from,
            user_name.to_string(),
            api_key.to_string(),
            api_url.to_string(),
        );

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

    // Mobile Data
    pub async fn send_mobiledata_async(
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
