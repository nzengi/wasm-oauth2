use reqwest::Client;
use crate::token::access_token::AccessToken;
use crate::error::OAuth2Error;
use serde_json::json;

pub async fn fetch_token(code: String) -> Result<AccessToken, OAuth2Error> {
    let client = Client::new();
    let params = json!({
        "code": code,
        "grant_type": "authorization_code",
    });

    let res = client.post("https://example.com/token")
        .json(&params)
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let token: AccessToken = response.json().await.unwrap();
                Ok(token)
            } else {
                Err(OAuth2Error::HttpError(response.status()))
            }
        },
        Err(err) => Err(OAuth2Error::RequestError(err.to_string()))
    }
}
