use crate::models::models::ResultFetchTransactionAirtimeMessage;
use crate::util::util::build_headers;
use reqwest::StatusCode;

pub async fn find_airtime_transaction_status_async(
    transaction_id: String,
    user_name: String,
    api_key: String,
    api_url: String,
) -> std::result::Result<ResultFetchTransactionAirtimeMessage, String> {
    let params = [("username", user_name), ("transactionId", transaction_id)];

    let client = reqwest::Client::new();
    let res = client
        .get(api_url)
        .headers(build_headers(api_key))
        .form(&params)
        .send()
        .await;

    match res {
        Err(e) => {
            return Err(e.to_string());
        }
        Ok(response) => match response.status() {
            StatusCode::CREATED => {
                match response
                    .json::<ResultFetchTransactionAirtimeMessage>()
                    .await
                {
                    Ok(result_message) => {
                        // Handle success case
                        return Ok(result_message);
                    }
                    Err(_err) => {
                        // Handle error case
                        return Err(_err.to_string());
                    }
                }
            }
            s => {
                let mut _x = String::from("Request failed processing, status code: ");
                _x.push_str(&s.to_string());
                return Err(_x.to_string());
            }
        },
    };
}
