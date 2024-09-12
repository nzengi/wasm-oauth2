use std::fmt;

#[derive(Debug)]
pub enum OAuth2Error {
    RequestError(String),
    HttpError(reqwest::StatusCode),
    TokenError(String),
}

impl fmt::Display for OAuth2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OAuth2Error::RequestError(err) => write!(f, "Request error: {}", err),
            OAuth2Error::HttpError(status) => write!(f, "HTTP error: {}", status),
            OAuth2Error::TokenError(err) => write!(f, "Token error: {}", err),
        }
    }
}

impl std::error::Error for OAuth2Error {}
