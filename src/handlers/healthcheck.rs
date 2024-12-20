use super::message::Response;
use axum::{debug_handler, extract::State, Json};

#[debug_handler]
pub async fn healthcheck() -> Json<Response<String>> {
    Json(Response {
        code: 200,
        result: "healthy".to_string(),
    })
}
