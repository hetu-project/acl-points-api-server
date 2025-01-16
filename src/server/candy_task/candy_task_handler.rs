use crate::database::services::candy_task;
use crate::{
    app::SharedState,
    common::error::{AppError, AppResult},
    server::middlewares::AuthToken,
};
use axum::{debug_handler, extract::State, Json};
use rand::Rng;

#[debug_handler]
pub async fn shake_candy(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    let attempts = state.store.get_user_attempts(claim.sub.as_str()).await?;

    let task_rule = candy_task::get_candy_task().await?.rule;

    if attempts >= task_rule.max_attempts_per_day {
        return Err(AppError::CustomError(
            "You have reached the maximum attempts for today.".into(),
        ));
    }

    let reward = rand::thread_rng().gen_range(task_rule.reward_min..=task_rule.reward_max);

    state.store.record_user_attempt(claim.sub, reward).await?;

    Ok(Json(serde_json::json!({
        "result": {
            "reward": reward,
            "count" : attempts,
            "limit" : task_rule.max_attempts_per_day
        }
    })))
}

#[debug_handler]
pub async fn get_shake_times(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    let attempts = state.store.get_user_attempts(claim.sub.as_str()).await?;

    let task_rule = candy_task::get_candy_task().await?.rule;

    Ok(Json(serde_json::json!({
    "result" : {
        "count" : attempts,
        "limit" : task_rule.max_attempts_per_day
    }
    })))
}

#[debug_handler]
pub async fn get_candy_count(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    let count = state.store.get_user_candy_count(claim.sub.as_str()).await?;

    Ok(Json(serde_json::json!({
    "result" : {
        "count" : count,
    }
    })))
}
