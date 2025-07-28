use axum::{response::IntoResponse, Json};
use serde_json::Value;
use std::sync::Arc;
use tower_sessions::Session;

pub async fn login(session: Session, Json(payload): Json<Value>) -> impl IntoResponse {
    match login_core(&payload, &session).await {
        Ok(user_id) => {
            println!("[/login] Logged in user_id: {}", user_id);
            Json(serde_json::json!({"status": "logged_in"}))
        }
        Err(e) => {
            println!("[/login] Login failed: {}", e);
            Json(serde_json::json!({"status": "error", "error": e}))
        }
    }
}

// The core logic, testable without Axum
pub async fn login_core(payload: &Value, session: &Session) -> Result<i32, String> {
    let username = payload
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let password = payload
        .get("password")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // Replace this with real credential checking!
    if username == "test" && password == "password" {
        let user_id = 42;
        session
            .insert("user_id", user_id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(user_id)
    } else {
        Err("Invalid credentials".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tower_sessions::{MemoryStore, Session};

    #[tokio::test]
    async fn test_login_success() {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(None, store.clone(), None);
        let payload = json!({"username": "test", "password": "password"});
        let result = login_core(&payload, &session).await;
        assert_eq!(result, Ok(42));
        let user_id: Option<i32> = session.get("user_id").await.unwrap();
        assert_eq!(user_id, Some(42));
    }

    #[tokio::test]
    async fn test_login_failure() {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(None, store.clone(), None);
        let payload = json!({"username": "wrong", "password": "bad"});
        let result = login_core(&payload, &session).await;
        assert!(result.is_err());
        let user_id: Option<i32> = session.get("user_id").await.unwrap();
        assert_eq!(user_id, None);
    }
}
