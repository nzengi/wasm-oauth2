use crate::wasm_oauth2::OAuth2Client;

fn main() {
    let client = OAuth2Client::new(
        "your_client_id".to_string(),
        "your_client_secret".to_string(),
        "https://github.com/login/oauth/authorize".to_string(),
        "https://github.com/login/oauth/access_token".to_string(),
        "https://yourapp.com/callback".to_string(),
    );

    let auth_url = client.authorize();
    println!("GitHub OAuth2 Yetkilendirme URL'si: {}", auth_url);
}
