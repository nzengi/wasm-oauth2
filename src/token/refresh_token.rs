pub struct RefreshToken {
    pub refresh_token: String,
}

impl RefreshToken {
    pub fn new(refresh_token: String) -> RefreshToken {
        RefreshToken { refresh_token }
    }

    pub fn refresh_access_token(&self) -> Option<String> {
        // Refresh token ile access token yenileme işlemi
        Some("new_access_token".to_string())
    }
}
