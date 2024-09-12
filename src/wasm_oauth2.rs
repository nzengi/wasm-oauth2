use serde::{Deserialize, Serialize};
use reqwest::Client;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::console;
use js_sys::Promise;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<u64>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}

#[wasm_bindgen]
pub struct OAuth2Client {
    client_id: String,
    client_secret: String,
    auth_url: String,
    token_url: String,
    redirect_url: String,
    http_client: Client,
}

#[wasm_bindgen]
impl OAuth2Client {
    #[wasm_bindgen(constructor)]
    pub fn new(
        client_id: String,
        client_secret: String,
        auth_url: String,
        token_url: String,
        redirect_url: String,
    ) -> OAuth2Client {
        console_error_panic_hook::set_once();

        OAuth2Client {
            client_id,
            client_secret,
            auth_url,
            token_url,
            redirect_url,
            http_client: Client::new(),
        }
    }

    // Kullanıcıyı yetkilendirme için yönlendiren URL'yi oluşturur.
    pub fn authorize_url(&self, scope: String, state: Option<String>) -> String {
        let mut url = format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}",
            self.auth_url, self.client_id, self.redirect_url, scope
        );

        if let Some(state_value) = state {
            url.push_str(&format!("&state={}", state_value));
        }

        url
    }

    // Yetkilendirme kodunu Access Token'a çevirir.
    pub fn exchange_code(&self, code: String) -> Promise {
        let client_id = self.client_id.clone();
        let client_secret = self.client_secret.clone();
        let token_url = self.token_url.clone();
        let redirect_url = self.redirect_url.clone();
        let http_client = self.http_client.clone();

        future_to_promise(async move {
            let params = [
                ("client_id", client_id.as_str()),
                ("client_secret", client_secret.as_str()),
                ("grant_type", "authorization_code"),
                ("code", code.as_str()),
                ("redirect_uri", redirect_url.as_str()),
            ];

            let response = http_client
                .post(&token_url)
                .form(&params)
                .send()
                .await
                .map_err(|e| JsValue::from_str(&format!("Request failed: {:?}", e)))?;

            if !response.status().is_success() {
                console::error_1(&JsValue::from_str(&format!(
                    "HTTP Error: {}",
                    response.status()
                )));
                return Err(JsValue::from_str(&format!(
                    "HTTP Error: {}",
                    response.status()
                )));
            }

            let token_response: AccessTokenResponse = response
                .json()
                .await
                .map_err(|e| JsValue::from_str(&format!("Failed to parse JSON: {:?}", e)))?;

            console::log_1(&JsValue::from_str(&format!(
                "Access token: {:?}",
                token_response
            )));

            Ok(JsValue::from_serde(&token_response).unwrap())
        })
    }
}
