use crate::models::user::User;
use crate::services::user_service::UserService;
use axum::{Extension, Json};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn create(
    Extension(user_service): Extension<Arc<dyn UserService>>,
    Json(payload): Json<CreateUserRequest>,
) -> Json<serde_json::Value> {
    match user_service
        .create_user(payload.name, payload.email, payload.password)
        .await
    {
        Ok(user) => Json(serde_json::json!({"status": "ok", "user_id": user.id})),
        Err(e) => Json(serde_json::json!({"status": "error", "error": e})),
    }
}
