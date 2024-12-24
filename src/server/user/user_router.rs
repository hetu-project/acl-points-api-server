use super::user_handler::{get_user_info, update_user_address};
use crate::app::SharedState;
use crate::server::middlewares;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

pub fn user_router(state: SharedState) -> Router<SharedState> {
    Router::new()
        .route("/info", get(get_user_info))
        .route("/info/address", post(update_user_address))
        .layer(middleware::from_fn_with_state(
            state,
            middlewares::auth_middleware,
        ))
}
