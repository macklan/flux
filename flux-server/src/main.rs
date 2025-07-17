mod websocket;
use futures::StreamExt;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Mutex};
use websocket::{Message, WebSocketClient}; // Adjust import as needed

type ClientList = Arc<Mutex<HashSet<mpsc::Sender<Message>>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;

    let clients: ClientList = Arc::new(Mutex::new(HashSet::new()));

    // {
    //     let clients = clients.clone();
    //     println!("clients: {:?}", clients);
    //     tokio::spawn(async move {
    //         println!("connecting to redis");
    //         let redis_client = redis::Client::open("redis://127.0.0..1/").unwrap();
    //         let redis_connection = redis_client.get_async_connection().await.unwrap();
    //         let mut redis_pubsub = redis_connection.into_pubsub();

    //         redis_pubsub.subscribe("room:general").await.unwrap();

    //         while let Some(msg) = redis_pubsub.on_message().next().await {
    //             let payload: String = msg.get_payload().unwrap();
    //             println!("got payload: {}", payload);
    //             let message = Message::Protocol(serde_json::from_str(&payload).unwrap());
    //             println!("message: {:?}", message);
    //             let clients = clients.lock().await;
    //             for tx in clients.iter() {
    //                 let _ = tx.send(message.clone()).await;
    //             }
    //             println!("GOT MESSAGE IN A ROOM: {}", payload);
    //         }
    //     });
    // }

    while let Ok((stream, addr)) = listener.accept().await {
        println!("Accepted connection from {}", addr);
        tokio::spawn(async move {
            if let Err(e) = websocket::handle_client(stream).await {
                eprintln!("Error in client handler: {}", e);
            }
        });
    }
    Ok(())
}
