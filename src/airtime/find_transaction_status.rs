use crate::models::ResultFetchTransactionAirtimeMessage;
use crate::utils::build_headers;
use reqwest::StatusCode;

pub async fn find_airtime_transaction_status_async(
    transaction_id: String,
    user_name: String,
    api_key: String,
    api_url: String,
) -> std::result::Result<Option<ResultFetchTransactionAirtimeMessage>, reqwest::Error> {
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
            return Err(e);
        }
        Ok(response) => match response.status() {
            StatusCode::CREATED => {
                let result_message = response
                    .json::<ResultFetchTransactionAirtimeMessage>()
                    .await?;

                return Ok(Some(result_message));
            }
            s => {
                return Ok(None);
            }
        },
    };
}
