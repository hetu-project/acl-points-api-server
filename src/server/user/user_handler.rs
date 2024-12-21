use crate::app::SharedState;
use crate::server::middlewares::AuthToken;
use axum::{debug_handler, extract::State, Json};

#[debug_handler]
pub async fn get_user_info(
    State(state): State<SharedState>,
    AuthToken(user): AuthToken,
) -> Json<serde_json::Value> {
    let client = state.jwt_handler.clone();
    let claim = client.decode_token(user).unwrap();

    Json(serde_json::json!({"name": claim.name,"email": claim.email}))
}
