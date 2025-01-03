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
    let user_rep = UserResponse::from(user);

    Ok(Json(serde_json::json!({
    "result": user_rep
    })))
}

#[debug_handler]
pub async fn get_user_count(
    State(state): State<SharedState>,
    AuthToken(_user): AuthToken,
) -> AppResult<Json<serde_json::Value>> {
    let count = state.store.count_total_users().await?;

    Ok(Json(serde_json::json!({
    "result": CountResponse{count}
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

#[debug_handler]
pub async fn get_user_invites(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    let invite_count = state
        .store
        .count_invited_users_by_email(claim.email.as_ref())
        .await?;

    tracing::info!("sub: {:?}", claim.sub);
    let point = match state.store.get_user_points(claim.sub.as_ref()).await {
        Ok(v) => v as u64,
        Err(e) => return Err(e),
    };

    Ok(Json(serde_json::json!({
    "result": PointsResponse{point,invite_count}
    })))
}
