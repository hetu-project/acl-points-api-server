use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, RedirectUrl, Scope, TokenResponse, TokenUrl,
};

const TOKEN_ENDPOINT: &str = "https://www.googleapis.com/oauth2/v3/token";
const USERINFO_ENDPOINT: &str = "https://www.googleapis.com/oauth2/v3/userinfo";
const AUTH_ENDPOINT: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const CLIENT_ID: &str = "";
const CLIENT_SECRET: &str = "";
const REDIRECT_URI: &str = "http://localhost:8080/auth/callback";

pub fn oauth_client() -> BasicClient {
    BasicClient::new(
        ClientId::new(CLIENT_ID.to_string()),
        Some(ClientSecret::new(CLIENT_SECRET.to_string())),
        AuthUrl::new(AUTH_ENDPOINT.to_string()).expect("Invalid auth URL"),
        Some(TokenUrl::new(TOKEN_ENDPOINT.to_string()).expect("Invalid token URL")),
    )
    .set_redirect_uri(RedirectUrl::new(REDIRECT_URI.to_string()).expect("Invalid redirect URL"))
}
