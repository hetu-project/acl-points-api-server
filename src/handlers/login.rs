use super::message::*;
use crate::common::consts;
use crate::server::app::SharedState;
use axum::{debug_handler, extract::State, Json};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use reqwest::Client;

use axum::extract::Query;

//https://www.googleapis.com/oauth2/v3/token
//https://oauth2.googleapis.com/token
const TOKEN_ENDPOINT: &str = "https://www.googleapis.com/oauth2/v3/token";
const USERINFO_ENDPOINT: &str = "https://www.googleapis.com/oauth2/v3/userinfo";
const AUTH_ENDPOINT: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const CLIENT_ID: &str = "";
const CLIENT_SECRET: &str = "";
//const REDIRECT_URI: &str = "http://localhost:8080/auth/callback";
const REDIRECT_URI: &str = "https://nftkash.xyz/auth/callback";
//const REDIRECT_URI: &str = "https://agent-dapp-extension.vercel.app/sign";

fn oauth_client(redirect_uri: String) -> BasicClient {
    BasicClient::new(
        ClientId::new(CLIENT_ID.to_string()),
        Some(ClientSecret::new(CLIENT_SECRET.to_string())),
        AuthUrl::new(consts::AUTH_ENDPOINT.to_string()).expect("Invalid auth URL"),
        Some(TokenUrl::new(consts::TOKEN_ENDPOINT.to_string()).expect("Invalid token URL")),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri).expect("Invalid redirect URL"))
}

#[debug_handler]
pub async fn auth_token(
    State(state): State<SharedState>,
    //Query(params): Query<OAuthCallbackParams>,
    Json(params): Json<OAuthParams>,
) -> Json<serde_json::Value> {
    tracing::info!("params: {:?}", params);

    let client = oauth_client(params.redirect_uri);
    //let client = state.0.read().await.oauth;

    let token_result = client
        .exchange_code(AuthorizationCode::new(params.code))
        .request_async(async_http_client)
        .await;
    tracing::info!("----{:?}", token_result);

    match token_result {
        Ok(token) => {
            let access_token = token.access_token().secret();
            tracing::info!("Access Token: {:?}", access_token);

            //Json(serde_json::json!({
            //    "access_token": access_token,
            //    "message": "Successfully authenticated"
            //}))
            let client = Client::new();

            let user_info_response = client
                .get(consts::USERINFO_ENDPOINT)
                .bearer_auth(&access_token)
                .send()
                .await
                .expect("Failed to get user info");

            if !user_info_response.status().is_success() {
                println!("{:?}", "abc");
            }

            let user_info: UserInfo = user_info_response
                .json()
                .await
                .expect("Failed to parse user info");
            println!("{:?}", user_info);

            let secret = state.0.read().await.jwt_handler.clone();
            let token: String = secret.create_token(&user_info.name, &user_info.email);

            return Json(serde_json::json!({
                "code": 200,
                "result": {
                    "access_token": token,
                    //"access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c",
                    "user_info": {
                        "name" : user_info.name,
                        "email" : user_info.email
                    }
                }
            }));
        }
        Err(err) => {
            eprintln!("Token exchange failed: {}", err);
            //(StatusCode::BAD_REQUEST, "Failed to authenticate").into_response()
            return Json(serde_json::json!({
                "code": 30001,
                "result": {
                    "error": "some error",
                }
            }));
        }
    }
}
