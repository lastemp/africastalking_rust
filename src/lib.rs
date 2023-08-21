pub mod models;
mod sms;
mod utils;

use models::{ResultSmsMessage, SmsMessage};

const SMS_URL_SANDBOX: &str = "https://api.sandbox.africastalking.com/version1/messaging";
const SMS_URL_PROD: &str = "https://api.africastalking.com/version1/messaging";

#[derive(Debug)]
pub struct AfricasTalking {
    user_name: String,
    api_key: String,
    sms_url: String,
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
            "sandbox" //String::from("sandbox")
        } else {
            "prod" //String::from("prod")
        };

        let sms_url: String = match _env {
            //"sandbox" => SMS_URL_SANDBOX.to_string(),
            "prod" => SMS_URL_PROD.to_string(),
            _ => SMS_URL_SANDBOX.to_string(),
        };

        let _env = _env.to_string();

        Ok(Self {
            user_name,
            api_key,
            sms_url,
            _env,
        })
    }

    pub async fn send_sms_message(
        &self,
        sms_message: SmsMessage,
    ) -> std::result::Result<Option<ResultSmsMessage>, reqwest::Error> {
        let _message = sms_message.get_message();
        let _to: String = sms_message.get_recipient();
        let user_name = &self.user_name;
        let api_key = &self.api_key;
        let api_url = &self.sms_url;

        let _output = sms::send_sms_message(
            _message,
            _to,
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
