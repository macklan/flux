# Video and Audio Streaming Application

## Overview
This project is a WebSocket server backend for a video and audio streaming web application. It utilizes Redis for pub/sub messaging and quick database operations, while MySQL is used for managing user accounts and call records. The frontend is built using TypeScript and the Svelte framework.

## Technology Stack
- **Backend**: Rust
  - WebSocket server for real-time communication
  - Redis for pub/sub messaging
  - MySQL for database management
- **Frontend**: TypeScript with Svelte
  - Interactive user interface for streaming

## Setup Instructions

### Backend
1. Navigate to the `flux-server` directory.
2. Ensure you have Rust installed. If not, install it from [rust-lang.org](https://www.rust-lang.org/).
3. Install the necessary dependencies by running:
   ```
   cargo build
   ```
4. Configure your Redis and MySQL connections in the code.
5. Start the backend server:
   ```
   cargo run
   ```

### Frontend
1. Navigate to the `flux-client` directory.
2. Ensure you have Node.js and npm installed. If not, install them from [nodejs.org](https://nodejs.org/).
3. Install the necessary dependencies by running:
   ```
   npm install
   ```
4. Start the frontend application:
   ```
   npm run dev
   ```

## Architecture Overview
The backend consists of several modules:
- **WebSocket**: Handles real-time communication with clients.
- **Redis**: Manages pub/sub messaging for efficient data distribution.
- **MySQL**: Handles user and call data storage and retrieval.
- **Models**: Defines the data structures used throughout the application.

## Usage Examples
- Connect to the WebSocket server from the frontend to start streaming audio and video.
- Use Redis pub/sub to manage notifications and updates in real-time.
- Interact with the MySQL database to manage user accounts and call records.

## Contribution Guidelines
Contributions are welcome! Please fork the repository and submit a pull request with your changes. Make sure to follow the coding standards and include tests for new features.