mod websocket;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("Async server listening on 127.0.0.1:7878");
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
