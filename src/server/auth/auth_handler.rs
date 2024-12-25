use super::auth_message::*;
use crate::{
    app::SharedState,
    common::{
        consts,
        error::{AppError, AppResult},
    },
    server::message::*,
};
use axum::{
    debug_handler,
    extract::{Query, State},
    Json,
};
use oauth2::{reqwest::async_http_client, AuthorizationCode, RedirectUrl, TokenResponse};
use reqwest::Client;

#[debug_handler]
pub async fn auth_token(
    State(state): State<SharedState>,
    Json(params): Json<OAuthParams>,
) -> AppResult<Json<serde_json::Value>> {
    tracing::info!("[auth_token] get params: {:?}", params);

    params.validate_items()?;

    let client = state
        .oauth
        .clone()
        .set_redirect_uri(RedirectUrl::new(state.config.auth.redirect_url.clone())?); //TODO from params?

    //TODO csf
    let token = client
        .exchange_code(AuthorizationCode::new(params.code.unwrap()))
        .request_async(async_http_client)
        .await
        .map_err(|_e| AppError::CustomError("failed to exchange code".to_string()))?;

    tracing::info!("[auth_token] exchange code get: {:?}", token);

    let access_token = token.access_token().secret();

    tracing::info!("[auth_token] Access Token: {:?}", access_token);

    let client = Client::new();

    let user_info_response = client
        .get(consts::USERINFO_ENDPOINT)
        .bearer_auth(&access_token)
        .send()
        .await
        .map_err(|_e| AppError::CustomError("failed to get user info".to_string()))?;

    if !user_info_response.status().is_success() {
        return Err(AppError::CustomError(
            "non user info in response".to_string(),
        ));
    }

    let user_info: UserInfo = user_info_response
        .json()
        .await
        .expect("Failed to parse user info");

    tracing::info!("[auth_token] get user info: {:?}", user_info);

    let created_user = match state.store.create_user(user_info.clone().into()).await {
        Ok(u) => u,
        Err(AppError::UserExisted(_)) => {
            tracing::info!("user has already existed, log in");
            state
                .store
                .get_user_by_email(user_info.email.as_ref())
                .await?
        }
        Err(e) => return Err(e),
    };

    tracing::info!("[auth_token] database  user info: {:?}", created_user);

    let secret = state.jwt_handler.clone();
    let token: String = secret.create_token(&user_info.name, &user_info.email);

    tracing::info!("[auth_token] jwt token: {:?}", token);

    return Ok(Json(serde_json::json!({
        "result": {
            "access_token": token,
            "user_info": UserResponse::from(created_user)
         }
    })));
}

#[debug_handler]
pub async fn callback_handler(
    State(_state): State<SharedState>,
    Query(params): Query<OAuthCallbackParams>,
) -> Json<serde_json::Value> {
    tracing::info!("auth params: {:?}", params);

    return Json(serde_json::json!({
        "result": {
            "code": params.code,
            "scope":params.scope,
            "authuser": params.authuser,
            "prompt": params.prompt ,
            "state": "authorization_code",
            "redirect_uri": "http://127.0.0.1:8080/auth/callback"
         }
    }));
}
