# Simple WebSocket Chat Backend

This is a basic WebSocket chat backend implemented in Rust. It allows multiple clients to connect and exchange messages in real-time.

## Features

- Handles WebSocket connections
- Broadcasts messages to all connected clients
- Lightweight and fast

## Prerequisites

1. Install [Rust](https://www.rust-lang.org/tools/install) (latest stable version).
2. Ensure `cargo` is available in your environment.

## Getting Started

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/simple-websocket-chat.git
   cd simple-websocket-chat
Build and run the project:

bash
Copy code
cargo run
The WebSocket server will start on ws://localhost:8080. You can connect to it using any WebSocket client.

Usage
Open a WebSocket client (e.g., websocat, browser console, or a WebSocket library).

Connect to the server:

bash
Copy code
websocat ws://localhost:8080
Send and receive messages in real-time.

