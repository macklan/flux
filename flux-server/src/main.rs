use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 512];

    match stream.read(&mut buffer).await {
        Ok(n) if n == 0 => {
            println!("Connection closed");
        }
        Ok(n) => {
            println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
            if let Err(e) = stream.write_all(b"Message received\n").await {
                eprintln!("Failed to write to client: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to read from client: {}", e),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("Async server listening on 127.0.0.1:7878");

    while let Ok((stream, addr)) = listener.accept().await {
        println!("Accepted connection from {}", addr);
        println!("TESTING: {}", addr.to_string());
        println!("WHAT IS GOING ON");
        tokio::spawn(async move {
            handle_client(stream).await;
        });
    }
    Ok(())
}
