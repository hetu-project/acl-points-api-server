use super::tasks_message::*;
use crate::{
    app::SharedState,
    common::error::{AppError, AppResult},
    server::middlewares::AuthToken,
};
use axum::{
    debug_handler,
    extract::{self, State},
    Json,
};

#[debug_handler]
pub async fn create_task(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
    extract::Json(req): extract::Json<Task>,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    let user = state.store.get_user_by_email(claim.email.as_ref()).await?;

    if user.role != "admin" {
        return Err(AppError::CustomError("not admin".into()));
    }

    state
        .store
        .add_task(req.name.as_str(), req.desc.as_str(), req.rule)
        .await?;

    Ok(Json(serde_json::json!({
        "result":"success"
    })))
}
