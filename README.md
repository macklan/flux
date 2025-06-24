# Flux 🌊

> A high-performance, Socket.io-inspired real-time communication library built in Rust

Flux is a modern real-time communication library that provides WebSocket-based messaging with advanced features like rooms, namespaces, and Redis-backed clustering. Built from the ground up in Rust for maximum performance and reliability.

## ✨ Features

- **Event-driven communication** - Emit and listen to custom events
- **Rooms & Namespaces** - Organize connections into logical groups  
- **Redis clustering** - Scale across multiple server instances
- **Automatic reconnection** - Built-in resilience for client connections
- **Binary & text messages** - Support for any data type
- **Authentication & middleware** - Secure and extensible connection handling
- **High performance** - Handle thousands of concurrent connections
- **Type-safe** - Leverage Rust's type system for reliable messaging

## 🚀 Quick Start

```rust
use flux::{Server, Socket};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new("127.0.0.1:3000").await?;
    
    server.on("connection", |socket: Socket| {
        println!("Client connected: {}", socket.id());
        
        socket.on("message", |data: String| {
            println!("Received: {}", data);
        });
        
        socket.emit("welcome", "Hello from Flux!");
    });
    
    server.listen().await?;
    Ok(())
}
```

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
flux = "0.1.0"
```

## 🏗️ Development Roadmap

### Phase 1: Foundation (Week 1)
- [ ] Project setup with Cargo workspace
- [ ] Basic HTTP server implementation
- [ ] WebSocket upgrade handshake
- [ ] Architecture documentation

### Phase 2: WebSocket Implementation (Week 2-3)
- [ ] WebSocket frame parsing and generation
- [ ] Control frame handling (ping/pong/close)
- [ ] Connection lifecycle management
- [ ] Heartbeat mechanism

### Phase 3: Core Protocol (Week 3-4)
- [ ] Custom message format design
- [ ] Event system implementation
- [ ] Acknowledgment mechanism
- [ ] Type-safe event handling

### Phase 4: Rooms & Namespaces (Week 5)
- [ ] Namespace isolation
- [ ] Room management system
- [ ] Broadcasting strategies
- [ ] Membership tracking

### Phase 5: Redis Integration (Week 6-7)
- [ ] Redis pub/sub client
- [ ] Cross-server messaging
- [ ] Shared session storage
- [ ] Cluster coordination

### Phase 6: Client Library (Week 8)
- [ ] Rust client implementation
- [ ] Automatic reconnection
- [ ] Event emission and handling
- [ ] Client-side room support

### Phase 7: Advanced Features (Week 9-10)
- [ ] Authentication hooks
- [ ] Middleware pipeline
- [ ] Rate limiting
- [ ] Compression support
- [ ] Binary data handling

### Phase 8: Testing & Polish (Week 11-12)
- [ ] Comprehensive test suite
- [ ] Load testing
- [ ] Performance benchmarks
- [ ] Documentation and examples

## 🏛️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Client App    │    │   Flux Server   │    │   Redis Cluster │
│                 │    │                 │    │                 │
│  ┌───────────┐  │    │  ┌───────────┐  │    │  ┌───────────┐  │
│  │ Flux      │  │◄──►│  │ WebSocket │  │◄──►│  │ Pub/Sub   │  │
│  │ Client    │  │    │  │ Handler   │  │    │  │ Adapter   │  │
│  └───────────┘  │    │  └───────────┘  │    │  └───────────┘  │
│                 │    │        │        │    │                 │
└─────────────────┘    │  ┌─────▼─────┐  │    └─────────────────┘
                       │  │ Event     │  │
                       │  │ System    │  │
                       │  └─────┬─────┘  │
                       │  ┌─────▼─────┐  │
                       │  │ Rooms &   │  │
                       │  │ Namespaces│  │
                       │  └───────────┘  │
                       └─────────────────┘
```

## 🛠️ Technology Stack

- **Runtime:** [Tokio](https://tokio.rs/) - Async runtime
- **WebSockets:** [tokio-tungstenite](https://github.com/snapview/tokio-tungstenite) - WebSocket implementation
- **Serialization:** [serde](https://serde.rs/) - JSON serialization
- **Redis:** [redis-rs](https://github.com/redis-rs/redis-rs) - Redis client
- **Concurrency:** [dashmap](https://github.com/xacrimon/dashmap) - Concurrent hash maps
- **Testing:** [criterion](https://github.com/bheisler/criterion.rs) - Benchmarking

## 🎯 Performance Goals

- **Connections:** 10,000+ concurrent connections per server
- **Latency:** <1ms message routing (local)
- **Throughput:** 100,000+ messages per second
- **Memory:** <100MB for 10,000 connections
- **Reliability:** 99.9% uptime

## 📚 Learning Resources

Each development phase focuses on specific Rust concepts:

- **Phase 1-2:** Async programming, HTTP protocols, WebSocket specification
- **Phase 3:** Trait design, serialization, type safety
- **Phase 4:** Concurrent data structures, memory management
- **Phase 5:** Distributed systems, Redis protocols
- **Phase 6:** Client library design, reconnection strategies
- **Phase 7:** Authentication, middleware patterns, performance optimization
- **Phase 8:** Testing strategies, benchmarking, documentation

## 🤝 Contributing

Flux is currently in active development. Contributions are welcome!

### Development Setup

1. Clone the repository:
```bash
git clone https://github.com/macklan/flux.git
cd flux
```

2. Install Rust (if not already installed):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Run tests:
```bash
cargo test
```

4. Run examples:
```bash
cargo run --example basic_server
```

### Project Structure

```
flux/
├── flux-core/          # Core protocol implementation
├── flux-server/        # Server implementation  
├── flux-client/        # Rust client library
├── flux-redis/         # Redis adapter
├── examples/           # Usage examples
├── benchmarks/         # Performance benchmarks
└── tests/             # Integration tests
```

## 📖 Documentation

- [API Documentation](https://docs.rs/flux) (Coming soon)
- [User Guide](./docs/guide.md) (Coming soon)
- [Architecture Overview](./docs/architecture.md) (Coming soon)
- [Performance Benchmarks](./docs/benchmarks.md) (Coming soon)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by [Socket.io](https://socket.io/) and its ecosystem
- Built with the amazing [Tokio](https://tokio.rs/) async runtime
- Thanks to the Rust community for excellent crates and documentation

---

**Status:** 🚧 In Development - Not ready for production use

**Current Version:** 0.1.0-alpha

**Rust Version:** 1.70+