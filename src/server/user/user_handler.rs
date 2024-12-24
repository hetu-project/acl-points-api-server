use crate::{
    app::SharedState,
    common::error::AppResult,
    server::{message::*, middlewares::AuthToken},
};
use axum::{debug_handler, extract::State, Json};

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
