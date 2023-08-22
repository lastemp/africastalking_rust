mod airtime;
pub mod models;
mod sms;
mod utils;

use models::{
    AirtimeInputRecipient, AirtimeInputRecipients, AirtimeMessage, ResultAirtimeMessage,
    ResultSmsMessage, SmsMessage,
};
use utils::parse_airtime_input_recipients;

//use serde_json::Result;

const SMS_URL_SANDBOX: &str = "https://api.sandbox.africastalking.com/version1/messaging";
const SMS_URL_PROD: &str = "https://api.africastalking.com/version1/messaging";
const DEFAULT_SENDER: &str = "AFRICASTKNG";
const AIRTIME_URL_SANDBOX: &str = "https://api.sandbox.africastalking.com/version1/airtime/send";
const AIRTIME_URL_PROD: &str = "https://api.africastalking.com/version1/airtime/send";

#[derive(Debug)]
pub struct AfricasTalking {
    user_name: String,
    api_key: String,
    sms_url: String,
    airtime_url: String,
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

        let _env = _env.to_string();

        Ok(Self {
            user_name,
            api_key,
            sms_url,
            airtime_url,
            _env,
        })
    }

    // SMS
    pub async fn send_message_async(
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

        let _output = sms::send_message_async(
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
    pub fn send_message(
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

        let _result = sms::send_message(
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
        let x = _recipients.len();
        if x > 0 {
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

        /*
        if let Ok(_recipients) = _result {
            println!("_recipients: {:?}", &_recipients);

            let _output = airtime::send_airtime_async(
                max_num_retry,
                _recipients,
                user_name.to_string(),
                api_key.to_string(),
                api_url.to_string(),
            );

            let _result = _output.await;

            return _result;
        } else if let Err(e) = _result {
            println!("{:?}", e);
            return Err(e);
        } else {
            println!("Unexpected error occured during processing");
        }
        */

        // convert _recipients to json
        let _recipients = parse_airtime_input_recipients(airtime_input_recipients);
        println!("_recipients: {:?}", &_recipients);

        let _output = airtime::send_airtime_async(
            max_num_retry,
            _recipients,
            user_name.to_string(),
            api_key.to_string(),
            api_url.to_string(),
        );

        let _result = _output.await;

        return _result;
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
