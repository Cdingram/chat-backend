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
   git clone https://github.com/Cdingram/chat-backend.git
   cd chat-backend
Build and run the project:

   ```bash
   cargo run

The WebSocket server will start on ws://127.0.0.1:8080/ws/. You can connect to it using any WebSocket client or the given index.html to test.
