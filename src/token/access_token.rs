use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<u64>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}

impl AccessToken {
    pub fn new(token: String, token_type: String) -> AccessToken {
        AccessToken {
            access_token: token,
            token_type,
            expires_in: None,
            refresh_token: None,
            scope: None,
        }
    }

    pub fn is_valid(&self) -> bool {
        // Token geçerliliğini kontrol eden mantık
        true
    }
}
