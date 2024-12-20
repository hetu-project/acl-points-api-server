use super::message::Response;
use axum::{debug_handler, extract::State, Json};

#[debug_handler]
pub async fn get_user_info() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "result": {
                "name" : "abc",
                "email" : "abc@gmail.com"
        }
    }))
}
