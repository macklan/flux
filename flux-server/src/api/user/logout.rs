use axum::{response::IntoResponse, Json};
use tower_sessions::Session;

pub async fn logout(session: Session) -> impl IntoResponse {
    session.remove::<i32>("user_id").await.unwrap();
    println!("[/logout] Logged out");
    Json(serde_json::json!({"status": "logged_out"}))
}
