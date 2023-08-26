use africastalking_rust::models::models::{
    BulkSmsMessage, CreateSubscriptionsMessage, DeleteSubscriptionMessage, FetchSmsMessage,
    FetchSubscriptionsMessage, PremiumSmsMessage,
};
use africastalking_rust::AfricasTalking;

pub async fn test_send_bulk_message_async(
    user_name: String,
    api_key: String,
    phone_number: String,
) {
    let _result = AfricasTalking::new(user_name, api_key);

    if let Ok(africas_talking) = _result {
        let _message = String::from("Have a wonderful time!");
        let _to = phone_number;
        let _from = None;
        let _enqueue = None;

        let _result = BulkSmsMessage::new(_message, _to, _from, _enqueue);
        if let Ok(sms_message) = _result {
            let _output = africas_talking.send_bulk_message_async(sms_message);
            let _result = _output.await;
            if let Ok(result_message) = _result {
                println!("result_message: {:?}", result_message);
                if let Some(result_sms) = result_message {
                    println!("result_sms: {:?}", result_sms);
                } else if let None = result_message {
                    println!("None");
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
    } else if let Err(e) = _result {
        println!("{:?}", e);
    } else {
        println!("Unexpected error occured during processing");
    }
}

pub fn test_send_bulk_message(user_name: String, api_key: String, phone_number: String) {
    let _result = AfricasTalking::new(user_name, api_key);

    if let Ok(africas_talking) = _result {
        let _message = String::from("Have a blessed time!");
        let _to = phone_number;
        let _from = None;
        let _enqueue = None;

        let _result = BulkSmsMessage::new(_message, _to, _from, _enqueue);
        if let Ok(sms_message) = _result {
            let _result = africas_talking.send_bulk_message(sms_message);
            if let Ok(result_message) = _result {
                println!("result_message: {:?}", result_message);
                if let Some(result_sms) = result_message {
                    println!("result_sms: {:?}", result_sms);
                } else if let None = result_message {
                    println!("None");
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
    } else if let Err(e) = _result {
        println!("{:?}", e);
    } else {
        println!("Unexpected error occured during processing");
    }
}

pub async fn test_send_premium_message_async(
    user_name: String,
    api_key: String,
    phone_number: String,
) {
    let _result = AfricasTalking::new(user_name, api_key);

    if let Ok(africas_talking) = _result {
        let _message = String::from("Have a wonderful time!");
        let _to = phone_number;
        let _from = String::from("kka7");
        let _keyword = None;
        let _enqueue = None;
        let link_id = None;
        let retry_duration_in_hours = None;
        let request_id = None;

        let _result = PremiumSmsMessage::new(
            _message,
            _to,
            _from,
            _keyword,
            _enqueue,
            link_id,
            retry_duration_in_hours,
            request_id,
        );
        if let Ok(sms_message) = _result {
            let _output = africas_talking.send_premium_message_async(sms_message);
            let _result = _output.await;
            if let Ok(result_message) = _result {
                println!("result_message: {:?}", result_message);
                if let Some(result_sms) = result_message {
                    println!("result_sms: {:?}", result_sms);
                } else if let None = result_message {
                    println!("None");
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
    } else if let Err(e) = _result {
        println!("{:?}", e);
    } else {
        println!("Unexpected error occured during processing");
    }
}

pub async fn test_fetch_sms_messages_async(
    user_name: String,
    api_key: String,
    phone_number: String,
) {
    let _result = AfricasTalking::new(user_name, api_key);

    if let Ok(africas_talking) = _result {
        let last_received_id = None;

        let _result = FetchSmsMessage::new(last_received_id);
        if let Ok(fetch_sms_message) = _result {
            let _output = africas_talking.fetch_sms_messages_async(fetch_sms_message);
            let _result = _output.await;
            if let Ok(result_message) = _result {
                println!("result_message: {:?}", result_message);
                if let Some(result_sms) = result_message {
                    println!("result_sms: {:?}", result_sms);
                } else if let None = result_message {
                    println!("None");
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
    } else if let Err(e) = _result {
        println!("{:?}", e);
    } else {
        println!("Unexpected error occured during processing");
    }
}

pub async fn test_create_sms_subscriptions_async(
    user_name: String,
    api_key: String,
    phone_number: String,
) {
    let _result = AfricasTalking::new(user_name, api_key);

    if let Ok(africas_talking) = _result {
        let short_code = String::from("1234");
        let _keyword = String::from("test");

        let _result = CreateSubscriptionsMessage::new(short_code, _keyword, phone_number);
        if let Ok(subscriptions_message) = _result {
            let _output = africas_talking.create_sms_subscriptions_async(subscriptions_message);
            let _result = _output.await;
            if let Ok(result_message) = _result {
                println!("result_message: {:?}", result_message);
                if let Some(result_sms) = result_message {
                    println!("result_sms: {:?}", result_sms);
                } else if let None = result_message {
                    println!("None");
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
    } else if let Err(e) = _result {
        println!("{:?}", e);
    } else {
        println!("Unexpected error occured during processing");
    }
}

pub async fn test_fetch_sms_subscriptions_async(
    user_name: String,
    api_key: String,
    phone_number: String,
) {
    let _result = AfricasTalking::new(user_name, api_key);

    if let Ok(africas_talking) = _result {
        let short_code = String::from("1234");
        let _keyword = String::from("test");
        let last_received_id = None;

        let _result = FetchSubscriptionsMessage::new(short_code, _keyword, last_received_id);
        if let Ok(subscriptions_message) = _result {
            let _output = africas_talking.fetch_sms_subscriptions_async(subscriptions_message);
            let _result = _output.await;
            if let Ok(result_message) = _result {
                println!("result_message: {:?}", result_message);
                if let Some(result_sms) = result_message {
                    println!("result_sms: {:?}", result_sms);
                } else if let None = result_message {
                    println!("None");
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
    } else if let Err(e) = _result {
        println!("{:?}", e);
    } else {
        println!("Unexpected error occured during processing");
    }
}

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
                if let Some(result_sms) = result_message {
                    println!("result_sms: {:?}", result_sms);
                } else if let None = result_message {
                    println!("None");
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
    } else if let Err(e) = _result {
        println!("{:?}", e);
    } else {
        println!("Unexpected error occured during processing");
    }
}
