# Video and Audio Streaming Application

This project is a WebSocket server-based backend for a video and audio streaming web application, utilizing Redis for pub/sub functionality and MySQL for managing calls and users. The frontend is built using Svelte and TypeScript.

## Technology Stack

- **Backend**: Rust
  - WebSocket server for real-time communication
  - Redis for pub/sub messaging
  - MySQL for database management
- **Frontend**: TypeScript with Svelte
  - Interactive user interface for streaming

## Project Structure

```
video-audio-streaming-app
├── flux-server
│   ├── src
│   │   ├── main.rs          # Entry point for the Rust backend
│   ├── Cargo.toml           # Rust project configuration
│   └── README.md            # Backend documentation
├── flux-client
│   ├── src
│   │   ├── main.ts          # Entry point for the TypeScript frontend
│   │   ├── App.svelte       # Main Svelte component
│   │   └── components
│   │       └── StreamPlayer.svelte # Streaming player component
│   ├── package.json         # Frontend project configuration
│   ├── tsconfig.json        # TypeScript configuration
│   └── README.md            # Frontend documentation
└── README.md                # Main project documentation
```

## Installation

1. Clone the repository:
   ```
   git clone <repository-url>
   cd video-audio-streaming-app
   ```

2. Set up the backend:
   - Navigate to the `flux-server` directory.
   - Install Rust and dependencies:
     ```
     cargo build
     ```

3. Set up the frontend:
   - Navigate to the `flux-client` directory.
   - Install Node.js dependencies:
     ```
     npm install
     ```

## Usage

- Start the backend server:
  ```
  cargo run
  ```

- Start the frontend application:
  ```
  npm run dev
  ```

## Contribution

Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.