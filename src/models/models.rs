use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct BulkSmsMessage {
    _message: String,       // The message to be sent.
    _to: String,            // A comma separated string of recipients’ phone numbers.
    _from: Option<String>,  // Your registered short code or alphanumeric, defaults to AFRICASTKNG.
    _enqueue: Option<bool>, // If enabled, the API will store the messages in a queue and send them out asynchronously.
}

impl BulkSmsMessage {
    pub fn new(
        _message: String,
        _to: String,
        _from: Option<String>,
        _enqueue: Option<bool>,
    ) -> Result<Self, String> {
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
            _enqueue,
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
    pub fn get_enqueue(&self) -> &Option<bool> {
        let _enqueue = &self._enqueue;
        _enqueue
    }
}

pub struct PremiumSmsMessage {
    _message: String,                    // The message to be sent.
    _to: String,                         // A comma separated string of recipients’ phone numbers.
    _from: String, // Your registered short code or alphanumeric, defaults to AFRICASTKNG.
    _keyword: Option<String>, // The keyword to be used for a premium service.
    _enqueue: Option<bool>, // If enabled, the API will store the messages in a queue and send them out asynchronously.
    link_id: Option<String>, // This is used for premium services to send OnDemand messages.
    retry_duration_in_hours: Option<u8>, // This specifies the number of hours your subscription message should be retried.
    request_id: Option<String>,          // This is a client specified request identifier.
}

impl PremiumSmsMessage {
    pub fn new(
        _message: String,
        _to: String,
        _from: String,
        _keyword: Option<String>,
        _enqueue: Option<bool>,
        link_id: Option<String>,
        retry_duration_in_hours: Option<u8>,
        request_id: Option<String>,
    ) -> Result<Self, String> {
        if _message.is_empty() || _message.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message is empty"));
        }

        if _to.is_empty() || _to.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message recipient is empty"));
        }

        if _from.is_empty() || _from.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message sender is empty"));
        }

        if let Some(_keyword) = &_keyword {
            if _keyword.is_empty() || _keyword.replace(" ", "").trim().len() == 0 {
                return Err(String::from("keyword is empty"));
            }
        }

        if let Some(link_id) = &link_id {
            if link_id.is_empty() || link_id.replace(" ", "").trim().len() == 0 {
                return Err(String::from("link id is empty"));
            }
        }

        if let Some(retry_duration_in_hours) = &retry_duration_in_hours {
            if *retry_duration_in_hours == 0 {
                return Err(String::from("Invalid retry_duration_in_hours"));
            }
        }

        if let Some(request_id) = &request_id {
            if request_id.is_empty() || request_id.replace(" ", "").trim().len() == 0 {
                return Err(String::from("request id is empty"));
            }
        }

        Ok(Self {
            _message,
            _to,
            _from,
            _keyword,
            _enqueue,
            link_id,
            retry_duration_in_hours,
            request_id,
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
    pub fn get_sender(&self) -> String {
        let _from = &self._from;
        _from.to_string()
    }
    pub fn get_keyword(&self) -> &Option<String> {
        let _keyword = &self._keyword;
        _keyword
    }
    pub fn get_enqueue(&self) -> &Option<bool> {
        let _enqueue = &self._enqueue;
        _enqueue
    }
    pub fn get_link_id(&self) -> &Option<String> {
        let link_id = &self.link_id;
        link_id
    }
    pub fn get_retry_duration_in_hours(&self) -> &Option<u8> {
        let retry_duration_in_hours = &self.retry_duration_in_hours;
        retry_duration_in_hours
    }
    pub fn get_request_id(&self) -> &Option<String> {
        let request_id = &self.request_id;
        request_id
    }
}

pub struct FetchSmsMessage {
    last_received_id: Option<u32>,
}

impl FetchSmsMessage {
    pub fn new(last_received_id: Option<u32>) -> Result<Self, String> {
        Ok(Self { last_received_id })
    }
    pub fn get_last_received_id(&self) -> Option<u32> {
        let last_received_id = &self.last_received_id;
        *last_received_id
    }
}

pub struct CreateSubscriptionsMessage {
    short_code: String,
    _keyword: String,
    phone_number: String,
}

impl CreateSubscriptionsMessage {
    pub fn new(short_code: String, _keyword: String, phone_number: String) -> Result<Self, String> {
        Ok(Self {
            short_code,
            _keyword,
            phone_number,
        })
    }
    pub fn get_short_code(&self) -> &String {
        let short_code = &self.short_code;
        short_code
    }
    pub fn get_keyword(&self) -> &String {
        let _keyword = &self._keyword;
        _keyword
    }
    pub fn get_phone_number(&self) -> &String {
        let phone_number = &self.phone_number;
        phone_number
    }
}

pub struct FetchSubscriptionsMessage {
    short_code: String,
    _keyword: String,
    last_received_id: Option<u32>,
}

impl FetchSubscriptionsMessage {
    pub fn new(
        short_code: String,
        _keyword: String,
        last_received_id: Option<u32>,
    ) -> Result<Self, String> {
        Ok(Self {
            short_code,
            _keyword,
            last_received_id,
        })
    }
    pub fn get_short_code(&self) -> &String {
        let short_code = &self.short_code;
        short_code
    }
    pub fn get_keyword(&self) -> &String {
        let _keyword = &self._keyword;
        _keyword
    }
    pub fn get_last_received_id(&self) -> Option<u32> {
        let last_received_id = &self.last_received_id;
        *last_received_id
    }
}

pub struct DeleteSubscriptionMessage {
    short_code: String,
    _keyword: String,
    phone_number: String,
}

impl DeleteSubscriptionMessage {
    pub fn new(short_code: String, _keyword: String, phone_number: String) -> Result<Self, String> {
        Ok(Self {
            short_code,
            _keyword,
            phone_number,
        })
    }
    pub fn get_short_code(&self) -> &String {
        let short_code = &self.short_code;
        short_code
    }
    pub fn get_keyword(&self) -> &String {
        let _keyword = &self._keyword;
        _keyword
    }
    pub fn get_phone_number(&self) -> &String {
        let phone_number = &self.phone_number;
        phone_number
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

        if _amount <= 0 {
            return Err(String::from("Invalid amount"));
        }

        if currency_code.is_empty() || currency_code.replace(" ", "").trim().len() == 0 {
            return Err(String::from("currency code is empty"));
        }

        if !currency_code.replace(" ", "").trim().len() == 3 {
            return Err(String::from("Invalid currency code"));
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
    max_num_retry: Option<u8>,
    _recipients: Vec<AirtimeRecipient>,
}

impl AirtimeMessage {
    pub fn new(
        max_num_retry: Option<u8>,
        _recipients: Vec<AirtimeRecipient>,
    ) -> Result<Self, String> {
        if let Some(max_num_retry) = &max_num_retry {
            if *max_num_retry == 0 {
                return Err(String::from("Invalid max num retry"));
            }
        }

        Ok(Self {
            max_num_retry,
            _recipients,
        })
    }
    pub fn get_max_num_retry(&self) -> Option<u8> {
        let max_num_retry = &self.max_num_retry;
        *max_num_retry
    }
    pub fn get_recipients(&self) -> &Vec<AirtimeRecipient> {
        let _recipients = &self._recipients;
        _recipients
    }
}

pub struct FindAirtimeMessage {
    transaction_id: String,
}

impl FindAirtimeMessage {
    pub fn new(transaction_id: String) -> Result<Self, String> {
        Ok(Self { transaction_id })
    }
    pub fn get_transaction_id(&self) -> &String {
        let transaction_id = &self.transaction_id;
        transaction_id
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

// Mobile Data request
#[derive(Debug)]
pub struct MobileDataRecipient {
    phone_number: String,
}

impl MobileDataRecipient {
    pub fn new(phone_number: String) -> Result<Self, String> {
        if phone_number.is_empty() || phone_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("phone number is empty"));
        }

        Ok(Self { phone_number })
    }
    pub fn get_phone_number(&self) -> String {
        let phone_number = &self.phone_number;
        phone_number.to_string()
    }
}

#[derive(Debug)]
pub struct MobileDataMessage {
    product_name: String,
    _recipients: Vec<MobileDataRecipient>,
    _quantity: u32,
    _unit: String,
    _validity: String,
    is_promo_bundle: bool,
}

impl MobileDataMessage {
    pub fn new(
        product_name: String,
        _recipients: Vec<MobileDataRecipient>,
        _quantity: u32,
        _unit: String,
        _validity: String,
        is_promo_bundle: bool,
    ) -> Result<Self, String> {
        if _quantity <= 0 {
            return Err(String::from("Invalid quantity"));
        }

        if _unit.is_empty() || _unit.replace(" ", "").trim().len() == 0 {
            return Err(String::from("unit is empty"));
        }

        if _unit.eq_ignore_ascii_case(&String::from("MB"))
            || _unit.eq_ignore_ascii_case(&String::from("GB"))
        { // valid
        } else {
            return Err(String::from("Invalid unit"));
        }

        if _validity.is_empty() || _validity.replace(" ", "").trim().len() == 0 {
            return Err(String::from("validity is empty"));
        }

        if _validity.eq_ignore_ascii_case(&String::from("Day"))
            || _unit.eq_ignore_ascii_case(&String::from("Week"))
            || _unit.eq_ignore_ascii_case(&String::from("BiWeek"))
            || _unit.eq_ignore_ascii_case(&String::from("Month"))
            || _unit.eq_ignore_ascii_case(&String::from("Quarterly"))
        { // valid
        } else {
            return Err(String::from("Incorrect validity"));
        }

        Ok(Self {
            product_name,
            _recipients,
            _quantity,
            _unit,
            _validity,
            is_promo_bundle,
        })
    }
    pub fn get_product_name(&self) -> String {
        let product_name = &self.product_name;
        product_name.to_string()
    }
    pub fn get_recipients(&self) -> &Vec<MobileDataRecipient> {
        let _recipients = &self._recipients;
        _recipients
    }
    pub fn get_quantity(&self) -> u32 {
        let _quantity = &self._quantity;
        *_quantity
    }
    pub fn get_unit(&self) -> String {
        let _unit = &self._unit;
        _unit.to_string()
    }
    pub fn get_validity(&self) -> String {
        let _validity = &self._validity;
        _validity.to_string()
    }
    pub fn get_is_promo_bundle(&self) -> bool {
        let is_promo_bundle = &self.is_promo_bundle;
        *is_promo_bundle
    }
}

pub struct FindMobileDataMessage {
    transaction_id: String,
}

impl FindMobileDataMessage {
    pub fn new(transaction_id: String) -> Result<Self, String> {
        Ok(Self { transaction_id })
    }
    pub fn get_transaction_id(&self) -> &String {
        let transaction_id = &self.transaction_id;
        transaction_id
    }
}

// Mobile Data response
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct MobileDataResponse {
    phoneNumber: Option<String>,
    provider: Option<String>,
    status: Option<String>,
    transactionId: Option<String>,
    value: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ResultMobileDataMessage {
    entries: Vec<MobileDataResponse>,
}

// Mobile Data input
#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
pub struct MobileDataInputRecipient {
    pub phoneNumber: String,
}

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
pub struct MobileDataInputRecipients {
    pub recipients: Vec<MobileDataInputRecipient>,
}

#[derive(Debug)]
pub enum ParamValue {
    Str(String),
    Int(u32),
    Bool(bool),
}

impl Serialize for ParamValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ParamValue::Str(s) => serializer.serialize_str(s),
            ParamValue::Int(i) => serializer.serialize_u32(*i),
            ParamValue::Bool(b) => serializer.serialize_bool(*b),
        }
    }
}

// Premium Sms Create Subscription response
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ResultPremiumSmsSubscriptionMessage {
    pub description: Option<String>,
    pub token: Option<String>,
}

// Premium Sms Fetch Subscription response
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct PremiumSmsSubscriptionsResponse {
    id: Option<u32>,
    phoneNumber: Option<String>,
    date: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ResultPremiumSmsFetchSubscriptionsMessage {
    responses: Vec<PremiumSmsSubscriptionsResponse>,
}

// Premium Sms Delete Subscription response
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ResultPremiumSmsDeleteSubscriptionMessage {
    status: Option<String>,
    description: Option<String>,
}

// Fetch Sms Messages response
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct FetchMessage {
    linkId: Option<String>,
    text: Option<String>,
    to: Option<String>,
    id: Option<u32>,
    date: Option<String>,
    from: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct FetchMessagesData {
    Messages: Vec<FetchMessage>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ResultFetchSmsMessages {
    SMSMessageData: FetchMessagesData,
}

// Fetch Transaction Airtime response
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ResultFetchTransactionAirtimeMessage {
    status: Option<String>,
}

// Fetch Transaction Mobile Data response
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct RequestMetadata {
    reason: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ProviderMetadata {
    recipientIsRegistered: Option<String>,
    recipientName: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct FetchTransactionMobileDataMessage {
    requestMetadata: RequestMetadata,
    sourceType: Option<String>,
    source: Option<String>,
    provider: Option<String>,
    destinationType: Option<String>,
    description: Option<String>,
    providerChannel: Option<String>,
    transactionFee: Option<String>,
    providerMetadata: ProviderMetadata,
    status: Option<String>,
    productName: Option<String>,
    category: Option<String>,
    transactionDate: Option<String>,
    destination: Option<String>,
    value: Option<String>,
    transactionId: Option<String>,
    creationTime: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ResultFetchTransactionMobileDataMessage {
    status: Option<String>,
    data: FetchTransactionMobileDataMessage,
    errorMessage: Option<String>,
}
