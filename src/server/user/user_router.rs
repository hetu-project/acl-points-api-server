use super::user_handler::{
    confirm_user_email, confirm_user_uid, get_user_count, get_user_info, get_user_invites,
    update_user_address,
};
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
        .route("/count", get(get_user_count))
        .route("/invites", get(get_user_invites))
        .route("/confirm/email", post(confirm_user_email))
        .route("/confirm/uid", post(confirm_user_uid))
        .layer(middleware::from_fn_with_state(
            state,
            middlewares::auth_middleware,
        ))
}
