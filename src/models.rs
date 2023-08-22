use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct SmsMessage {
    _message: String,
    _to: String,
    _from: Option<String>,
}

impl SmsMessage {
    pub fn new(_message: String, _to: String, _from: Option<String>) -> Result<Self, String> {
        if _message.is_empty() || _message.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message is empty"));
        }

        if _to.is_empty() || _to.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message recipient is empty"));
        }

        if let Some(_from) = &_from {
            if _from.is_empty() || _from.replace(" ", "").trim().len() == 0 {
                return Err(String::from("message sender is empty"));
            }
        }

        Ok(Self {
            _message,
            _to,
            _from,
        })
    }
    pub fn get_message(&self) -> String {
        let _message = &self._message;
        _message.to_string()
    }
    pub fn get_recipient(&self) -> String {
        let _to = &self._to;
        _to.to_string()
    }
    pub fn get_sender(&self) -> &Option<String> {
        let _from = &self._from;
        _from
    }
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct Recipients {
    messageId: Option<String>,
    number: Option<String>,
    statusCode: Option<u32>,
    status: Option<String>,
    cost: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct SMSMessageData {
    Message: Option<String>,
    Recipients: Vec<Recipients>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ResultSmsMessage {
    SMSMessageData: SMSMessageData,
}
// Airtime request
#[derive(Debug)]
pub struct AirtimeRecipient {
    phone_number: String,
    _amount: u32,
    currency_code: String,
}

impl AirtimeRecipient {
    pub fn new(phone_number: String, _amount: u32, currency_code: String) -> Result<Self, String> {
        if phone_number.is_empty() || phone_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("phone number is empty"));
        }

        if _amount == 0 {
            return Err(String::from("Invalid amount"));
        }

        if currency_code.is_empty() || currency_code.replace(" ", "").trim().len() == 0 {
            return Err(String::from("currency code is empty"));
        }
        Ok(Self {
            phone_number,
            _amount,
            currency_code,
        })
    }
    pub fn get_phone_number(&self) -> String {
        let phone_number = &self.phone_number;
        phone_number.to_string()
    }
    pub fn get_amount(&self) -> u32 {
        let _amount = &self._amount;
        *_amount
    }
    pub fn get_currency_code(&self) -> String {
        let currency_code = &self.currency_code;
        currency_code.to_string()
    }
}

#[derive(Debug)]
pub struct AirtimeMessage {
    max_num_retry: u8,
    _recipients: Vec<AirtimeRecipient>,
}

impl AirtimeMessage {
    pub fn new(max_num_retry: u8, _recipients: Vec<AirtimeRecipient>) -> Result<Self, String> {
        if max_num_retry >= 0 && max_num_retry <= 8 {
        } else {
            return Err(String::from("Invalid max num retry"));
        }

        // validate _recipients

        Ok(Self {
            max_num_retry,
            _recipients,
        })
    }
    pub fn get_max_num_retry(&self) -> u8 {
        let max_num_retry = &self.max_num_retry;
        *max_num_retry
    }
    pub fn get_recipients(&self) -> &Vec<AirtimeRecipient> {
        let _recipients = &self._recipients;
        _recipients
    }
}

// Airtime response
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct AirtimeResponse {
    phoneNumber: Option<String>,
    amount: Option<String>,
    discount: Option<String>,
    status: Option<String>,
    requestId: Option<String>,
    errorMessage: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ResultAirtimeMessage {
    errorMessage: Option<String>,
    numSent: Option<u32>,
    totalAmount: Option<String>,
    totalDiscount: Option<String>,
    responses: Vec<AirtimeResponse>,
}

// Airtime input
#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
pub struct AirtimeInputRecipient {
    pub phoneNumber: String,
    pub amount: String,
}

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
pub struct AirtimeInputRecipients {
    pub recipients: Vec<AirtimeInputRecipient>,
}
