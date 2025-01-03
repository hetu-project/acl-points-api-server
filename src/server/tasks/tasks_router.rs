use super::tasks_handler::create_task;
use crate::app::SharedState;
use crate::server::middlewares;
use axum::{middleware, routing::get, Router};

pub fn tasks_router(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/create", get(create_task))
        .layer(middleware::from_fn_with_state(
            state,
            middlewares::auth_middleware,
        ))
}
