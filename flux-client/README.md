# Flux Client Documentation for Video and Audio Streaming App

## Overview

This project is a web application designed for video and audio streaming. It consists of a Rust backend that handles WebSocket connections and manages data using Redis and MySQL, and a TypeScript frontend built with Svelte.

## Technology Stack

- **Frontend**: TypeScript, Svelte
- **Backend**: Rust
- **Database**: MySQL
- **Caching and Pub/Sub**: Redis

## Setup Instructions

### Prerequisites

- Node.js and npm installed
- TypeScript installed globally (optional)
- A running instance of MySQL
- A running instance of Redis

### Installation

1. Clone the repository:
   ```
   git clone <repository-url>
   cd flux/flux-client
   ```

2. Install dependencies:
   ```
   npm install
   ```

3. Configure your environment variables for connecting to the MySQL and Redis servers.

### Running the Application

To start the frontend application, run:
```
npm run dev
```

This will start the development server and you can access the application at `http://localhost:5000` (or the port specified in your configuration).

## Component Descriptions

- **App.svelte**: The main component that serves as the entry point for the application.
- **StreamPlayer.svelte**: A component responsible for rendering the video and audio player, handling playback controls, and managing streaming.

## Usage Examples

- To use the streaming player, import the `StreamPlayer` component in your Svelte files and pass the necessary props for stream management.

## Contribution Guidelines

Contributions are welcome! Please follow the standard Git workflow for submitting pull requests. Ensure that your code adheres to the project's coding standards and includes appropriate tests.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.