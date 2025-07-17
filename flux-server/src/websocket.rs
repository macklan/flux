use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use serde::{Deserialize, Serialize};
use serde_json;
use sha1::{Digest, Sha1};
use std::{pin, vec};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{self, Duration, Instant};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
enum ConnectionState {
    Open,
    Closed,
    Error(String),
}

pub struct WebSocketClient {
    stream: TcpStream,
    #[allow(dead_code)]
    id: Uuid,
    #[allow(dead_code)]
    connection_state: ConnectionState,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ProtocolMessage {
    Join {},
    Leave {},
    Message {},
}

#[derive(Clone, Debug)]
pub enum Message {
    Protocol(ProtocolMessage),
    Ping,
    Pong,
    Close(u16, String),
}

impl WebSocketClient {
    pub async fn new(stream: TcpStream) -> Self {
        let id = Uuid::new_v4();
        let connection_state = ConnectionState::Open;
        WebSocketClient {
            stream,
            id,
            connection_state,
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut last_pong = Instant::now();
        let mut ping_interval = time::interval(Duration::from_secs(10));
        ping_interval.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
        loop {
            tokio::select! {
                _ = ping_interval.tick() => {
                    self.send_message(&Message::Ping).await?;
                    if last_pong.elapsed() > Duration::from_secs(20) {
                        self.send_message(&Message::Close(1001, "Ping timeout".into())).await?;
                        break;
                    }
                }
            }

            match self.read_message().await? {
                Message::Protocol(msg) => {
                    println!("Received protocol message: {:?}", msg);
                    // Here you would handle each ProtocolMessage variant
                }
                Message::Ping => {
                    println!("PING");
                    self.send_message(&Message::Pong).await?;
                }
                Message::Pong => {
                    println!("PONG");
                    last_pong = Instant::now();
                }
                Message::Close(code, reason) => {
                    println!("CLOSE RECEIVED: code={}, reason={}", code, reason);
                    self.send_message(&Message::Close(1000, "".to_string()))
                        .await?;
                    break;
                }
            }
        }
        Ok(())
    }

    pub async fn send_message(
        &mut self,
        msg: &Message,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match msg {
            Message::Protocol(proto_msg) => {
                let json = serde_json::to_string(proto_msg)?;
                let mut frame = vec![0b1000_0001]; // FIN=1, opcode=1 (text)
                let bytes = json.as_bytes();
                frame.push(bytes.len() as u8);
                frame.extend_from_slice(bytes);
                self.stream.write_all(&frame).await?;
            }
            Message::Ping => {
                let frame = [0b1000_1001, 0x00];
                self.stream.write_all(&frame).await?;
            }
            Message::Pong => {
                let frame = [0b1000_1010, 0x00];
                self.stream.write_all(&frame).await?;
            }
            Message::Close(code, reason) => {
                let mut frame = vec![0b1000_1000];
                let reason_bytes = reason.as_bytes();
                let len = 2 + reason_bytes.len();
                frame.push(len as u8);
                frame.extend_from_slice(&code.to_be_bytes());
                frame.extend_from_slice(reason_bytes);
                self.stream.write_all(&frame).await?;
            }
        }
        Ok(())
    }

    pub async fn read_message(
        &mut self,
    ) -> Result<Message, Box<dyn std::error::Error + Send + Sync>> {
        let mut header = [0u8; 2];
        self.stream.read_exact(&mut header).await?;
        let opcode = header[0] & 0b0000_1111;
        let masked = header[1] & 0b1000_0000 != 0;
        let mut payload_len = (header[1] & 0b0111_1111) as usize;

        if payload_len == 126 {
            let mut ext = [0u8; 2];
            self.stream.read_exact(&mut ext).await?;
            payload_len = u16::from_be_bytes(ext) as usize;
        } else if payload_len == 127 {
            let mut ext = [0u8; 8];
            self.stream.read_exact(&mut ext).await?;
            payload_len = u64::from_be_bytes(ext) as usize;
        }

        let mut mask = [0u8; 4];
        if masked {
            self.stream.read_exact(&mut mask).await?;
        }

        let mut payload = vec![0u8; payload_len];
        self.stream.read_exact(&mut payload).await?;

        if masked {
            for i in 0..payload_len {
                payload[i] ^= mask[i % 4];
            }
        }

        match opcode {
            0x1 => {
                // Text frame: try to parse as ProtocolMessage
                match std::str::from_utf8(&payload) {
                    Ok(text) => match serde_json::from_str::<ProtocolMessage>(text) {
                        Ok(proto_msg) => Ok(Message::Protocol(proto_msg)),
                        Err(e) => {
                            eprintln!("Invalid protocol message: {}", e);
                            // Optionally, close connection or ignore
                            Ok(Message::Close(1002, "Invalid protocol message".to_string()))
                        }
                    },
                    Err(_) => Ok(Message::Close(1007, "Invalid UTF-8".to_string())),
                }
            }
            0x8 => {
                let (code, reason) = if payload_len >= 2 {
                    let code = u16::from_be_bytes([payload[0], payload[1]]);
                    let reason = if payload_len > 2 {
                        match std::str::from_utf8(&payload[2..]) {
                            Ok(s) => s.to_string(),
                            Err(_) => "<invalid utf8>".to_string(),
                        }
                    } else {
                        String::new()
                    };
                    (code, reason)
                } else {
                    (1000, String::new())
                };
                Ok(Message::Close(code, reason))
            }
            0x9 => Ok(Message::Ping),
            0xA => Ok(Message::Pong),
            _ => Ok(Message::Close(1003, "Unsupported frame type".to_string())),
        }
    }
}

pub async fn handle_client(
    mut stream: TcpStream,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let request = read_http_request(&mut stream).await?;
    if request.contains("Upgrade: websocket") {
        println!("WebSocket upgrade requested!");
        match parse_websocket_key(&request) {
            Some(key) => {
                let accept_key = generate_websocket_accept_key(&key);
                println!("Found key: {}", accept_key);
                let response = format!(
                    "HTTP/1.1 101 Switching Protocols\r\n\
                    Upgrade: websocket\r\n\
                    Connection: Upgrade\r\n\
                    Sec-WebSocket-Accept: {}\r\n\
                    \r\n",
                    accept_key
                );
                stream.write_all(response.as_bytes()).await?;
                let mut websocket = WebSocketClient::new(stream).await;
                websocket.run().await?;
            }
            None => {
                println!("No key");
            }
        }
    } else {
        println!("Not a WebSocket upgrade request.");
    }
    Ok(())
}

async fn read_http_request(
    stream: &mut TcpStream,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let mut buffer = [0u8; 512];
    let n = match stream.read(&mut buffer).await {
        Ok(n) if n == 0 => {
            println!("Connection closed");
            return Ok(String::new());
        }
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to read from client: {}", e);
            return Err(Box::new(e));
        }
    };
    Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
}

fn parse_websocket_key(request: &str) -> Option<String> {
    for line in request.lines() {
        if line.starts_with("Sec-WebSocket-Key") {
            if let Some(key) = line.splitn(2, ":").nth(1) {
                println!("Found key: {}", key);
                return Some(key.trim().to_string());
            }
        }
    }
    None
}

fn generate_websocket_accept_key(key: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(key.as_bytes());
    hasher.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    let result = hasher.finalize();
    STANDARD.encode(result)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_message_serialization() {
        let join = ProtocolMessage::Join {};
        let json = serde_json::to_string(&join).unwrap();
        assert_eq!(json, r#"{"type":"Join"}"#);

        let leave = ProtocolMessage::Leave {};
        let json = serde_json::to_string(&leave).unwrap();
        assert_eq!(json, r#"{"type":"Leave"}"#);
    }

    #[test]
    fn test_protocol_message_deserialization() {
        let json = r#"{"type":"Join"}"#;
        let msg: ProtocolMessage = serde_json::from_str(json).unwrap();
        match msg {
            ProtocolMessage::Join {} => {}
            _ => panic!("Expected Join"),
        }
        let json = r#"{"type":"Leave"}"#;
        let msg: ProtocolMessage = serde_json::from_str(json).unwrap();
        match msg {
            ProtocolMessage::Leave {} => {}
            _ => panic!("Expected Leave"),
        }
    }
}
