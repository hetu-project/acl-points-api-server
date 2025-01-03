use crate::database::services::candy_task;
use crate::{
    app::SharedState,
    common::error::{AppError, AppResult},
    server::middlewares::AuthToken,
};
use axum::{
    debug_handler,
    extract::{self, Query, State},
    Json,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct CandyTaskParams {}

#[debug_handler]
async fn shake_candy(
    State(state): State<SharedState>,
    Query(params): Query<CandyTaskParams>,
    AuthToken(user): AuthToken,
) -> AppResult<Json<serde_json::Value>> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    //TODO sub as uid?
    let attempts = state
        .store
        .get_user_attempts(claim.sub.clone(), "candy_task".into())
        .await?;

    let task_rule = candy_task::get_candy_task().await?.rule;

    if attempts >= task_rule.max_attempts_per_day {
        return Err(AppError::CustomError(
            "You have reached the maximum attempts for today.".into(),
        ));
    }

    let reward = rand::thread_rng().gen_range(task_rule.reward_min..=task_rule.reward_max);

    state
        .store
        .record_user_attempt(claim.sub, "candy_task".into(), reward)
        .await?;

    Ok(Json(serde_json::json!({
        "message": "You received a candy reward!",
        "reward": reward
    })))
}
