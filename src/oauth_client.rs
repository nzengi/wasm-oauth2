use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use serde::{Deserialize, Serialize};
use crate::token::access_token::AccessToken;
use crate::http::fetch_client::fetch_token;
use crate::error::OAuth2Error;

#[derive(Serialize, Deserialize)]
pub struct OAuth2Client {
    client_id: String,
    client_secret: String,
    auth_url: String,
    token_url: String,
    redirect_url: String,
}

impl OAuth2Client {
    pub fn new(client_id: String, client_secret: String, auth_url: String, token_url: String, redirect_url: String) -> OAuth2Client {
        OAuth2Client {
            client_id,
            client_secret,
            auth_url,
            token_url,
            redirect_url,
        }
    }

    pub fn authorize(&self) -> String {
        format!("{}?client_id={}&redirect_uri={}&response_type=code&scope=openid", 
            self.auth_url, self.client_id, self.redirect_url)
    }

    pub fn exchange_code(code: String) -> impl std::future::Future<Output = Result<AccessToken, OAuth2Error>> {
        fetch_token(code)
    }
}
