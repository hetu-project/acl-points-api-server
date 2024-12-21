use super::user_handler::get_user_info;
use crate::app::SharedState;
use crate::server::middlewares;
use axum::{middleware, routing::get, Router};

pub fn user_router(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/info", get(get_user_info))
        .layer(middleware::from_fn_with_state(
            state,
            middlewares::auth_middleware,
        ))
}
