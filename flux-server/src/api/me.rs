use axum::{response::IntoResponse, Json};
use tower_sessions::Session;

pub async fn me(session: Session) -> impl IntoResponse {
    if let Ok(Some(user_id)) = session.get::<i32>("user_id").await {
        println!("[/me] Authenticated user_id: {}", user_id);
        Json(serde_json::json!({"authenticated": true, "user_id": user_id}))
    } else {
        println!("[/me] Unauthenticated request");
        Json(serde_json::json!({"authenticated": false}))
    }
}
