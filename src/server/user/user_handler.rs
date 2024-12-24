use super::user_message::*;
use crate::{
    app::SharedState,
    common::error::AppResult,
    server::{message::*, middlewares::AuthToken},
};
use axum::{
    debug_handler,
    extract::{self, State},
    Json,
};

#[debug_handler]
pub async fn get_user_info(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    let user = state.store.get_user_by_email(claim.email.as_ref()).await?;

    Ok(Json(serde_json::json!({
    "result": UserResponse::from(user)
    })))
}

#[debug_handler]
pub async fn update_user_address(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
    extract::Json(req): extract::Json<UpdateAddressReq>,
) -> AppResult<Json<serde_json::Value>> {
    tracing::info!("get req: {:?}", req);

    req.validate_items()?;
    let user_addr = req.address.unwrap();

    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    state
        .store
        .update_user_address_by_email(claim.email.as_ref(), user_addr.as_ref())
        .await?;

    let user = state.store.get_user_by_email(claim.email.as_ref()).await?;

    Ok(Json(serde_json::json!({
    "result": UserResponse::from(user)
    })))
}
