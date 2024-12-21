use crate::app::SharedState;
use crate::common::consts;
use crate::server::message::*;
use axum::{
    debug_handler,
    extract::{Query, State},
    Json,
};
//use oauth2::{
//    basic::BasicClient,  AuthUrl,  ClientId,
//    ClientSecret,  TokenUrl,
//};
use oauth2::{reqwest::async_http_client, AuthorizationCode, RedirectUrl, TokenResponse};
use reqwest::Client;

//const CLIENT_ID: &str = "";
//const CLIENT_SECRET: &str = "";
////const REDIRECT_URI: &str = "https://nftkash.xyz/auth/callback";
//
//fn oauth_client(redirect_uri: String) -> BasicClient {
//    BasicClient::new(
//        ClientId::new(CLIENT_ID.to_string()),
//        Some(ClientSecret::new(CLIENT_SECRET.to_string())),
//        AuthUrl::new(consts::AUTH_ENDPOINT.to_string()).expect("Invalid auth URL"),
//        Some(TokenUrl::new(consts::TOKEN_ENDPOINT.to_string()).expect("Invalid token URL")),
//    )
//    .set_redirect_uri(RedirectUrl::new(redirect_uri).expect("Invalid redirect URL"))
//}

#[debug_handler]
pub async fn auth_token(
    State(state): State<SharedState>,
    //Query(params): Query<OAuthCallbackParams>,
    Json(params): Json<OAuthParams>,
) -> Json<serde_json::Value> {
    tracing::info!("params: {:?}", params);

    //let client = oauth_client(params.redirect_uri);

    let client = state.oauth.clone().set_redirect_uri(
        RedirectUrl::new(state.config.auth.redirect_url.clone()).expect("Invalid redirect URL"),
    );

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

            let secret = state.jwt_handler.clone();
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

#[debug_handler]
pub async fn callback_handler(
    State(state): State<SharedState>,
    Query(params): Query<OAuthCallbackParams>,
) -> Json<serde_json::Value> {
    tracing::info!("auth params: {:?}", params);

    let client = state.oauth.clone().set_redirect_uri(
        RedirectUrl::new(state.config.auth.redirect_url.clone()).expect("Invalid redirect URL"),
    );

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

            let secret = state.jwt_handler.clone();
            let token: String = secret.create_token(&user_info.name, &user_info.email);

            return Json(serde_json::json!({
                "code": 200,
                "result": {
                    "access_token": token,
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
