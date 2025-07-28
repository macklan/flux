mod api;
mod models;
mod providers;
mod redis;
mod schema;
mod services;
mod traits;
mod websocket;

use crate::models::create_models;
use crate::providers::MySQLProvider;
use crate::services::user_service::UserService;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let db_provider = MySQLProvider::new("mysql://root@localhost/flux");
    let user_service: Arc<dyn UserService> = Arc::new(db_provider);

    let api = api::Api::new("127.0.0.1:3001".parse().unwrap(), user_service);
    api.start().await;
}
