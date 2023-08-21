use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct SmsMessage {
    _message: String,
    _to: String,
}

impl SmsMessage {
    pub fn new(_message: String, _to: String) -> Result<Self, String> {
        if _message.is_empty() || _message.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message is empty"));
        }

        if _to.is_empty() || _to.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message recipient is empty"));
        }

        Ok(Self { _message, _to })
    }
    pub fn get_message(&self) -> String {
        let _message = &self._message;
        _message.to_string()
    }
    pub fn get_recipient(&self) -> String {
        let _to = &self._to;
        _to.to_string()
    }
}

#[derive(Deserialize, Debug)]
struct Recipients {
    messageId: Option<String>,
    number: Option<String>,
    statusCode: Option<u32>,
    status: Option<String>,
    cost: Option<String>,
}

#[derive(Deserialize, Debug)]
struct SMSMessageData {
    Message: Option<String>,
    Recipients: Vec<Recipients>,
}

#[derive(Deserialize, Debug)]
pub struct ResultSmsMessage {
    //Id: Option<String>,
    SMSMessageData: SMSMessageData,
}
