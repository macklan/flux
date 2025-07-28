use axum::{response::IntoResponse, Json};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn health() -> impl IntoResponse {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    Json(serde_json::json!({"status": "ok", "ts": now}))
}
