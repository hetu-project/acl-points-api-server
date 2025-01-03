use super::candy_task_handler::{get_candy_count, get_shake_times, shake_candy};
use crate::app::SharedState;
use crate::server::middlewares;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

pub fn candy_router(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/count", get(get_candy_count))
        .route("/attempt", get(get_shake_times))
        .route("/shark", post(shake_candy))
        .layer(middleware::from_fn_with_state(
            state,
            middlewares::auth_middleware,
        ))
}
