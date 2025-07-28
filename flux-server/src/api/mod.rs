mod health;
mod me;
mod user;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_sessions::{MemoryStore, SessionManagerLayer};

use crate::services::user_service::UserService;

pub struct Api {
    addr: SocketAddr,
    app: Router,
}

impl Api {
    pub fn new(addr: SocketAddr, user_service: Arc<dyn UserService>) -> Self {
        // Set up in-memory session store
        let store = MemoryStore::default();
        let session_layer = SessionManagerLayer::new(store).with_secure(false); // set to true for HTTPS

        let app = Router::new()
            .route("/health", get(health::health))
            .route("/login", post(user::login::login))
            .route("/logout", post(user::logout::logout))
            .route("/user/create", post(user::create::create))
            .route("/me", get(me::me))
            .layer(Extension(user_service))
            .layer(session_layer);

        Self { addr, app }
    }

    pub async fn start(self) {
        let listener = tokio::net::TcpListener::bind(&self.addr).await.unwrap();
        println!("listening on: {}", &self.addr);
        axum::serve(
            listener,
            self.app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap();
    }
}
