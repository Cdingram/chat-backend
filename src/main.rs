use actix::prelude::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use std::collections::HashMap;
use std::time::{Duration, Instant};

use uuid::Uuid;

/// Define message types
#[derive(Message)]
#[rtype(result = "()")]
struct ChatMessage(String);

#[derive(Message)]
#[rtype(result = "()")]
struct Connect {
    id: Uuid,
    addr: Recipient<ChatMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
struct Disconnect {
    id: Uuid,
}

/// Chat server manages connected clients and broadcasts messages
struct ChatServer {
    sessions: HashMap<Uuid, Recipient<ChatMessage>>,
}

impl ChatServer {
    fn new() -> Self {
        ChatServer {
            sessions: HashMap::new(),
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

/// Handle messages received by the server
impl Handler<Connect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        self.sessions.insert(msg.id, msg.addr);
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<ChatMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, _: &mut Context<Self>) {
        // Broadcast the message to all connected clients
        for session in self.sessions.values() {
            let _ = session.do_send(ChatMessage(msg.0.clone()));
        }
    }
}

/// Define WebSocket connection actor
struct ChatSession {
    id: Uuid,
    hb: Instant,
    addr: Addr<ChatServer>,
}

impl ChatSession {
    fn new(addr: Addr<ChatServer>) -> Self {
        ChatSession {
            id: Uuid::new_v4(),
            hb: Instant::now(),
            addr,
        }
    }

    /// Helper method to send periodic heartbeats
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::from_secs(5), |act, ctx| {
            // Check client heartbeats
            if Instant::now().duration_since(act.hb) > Duration::from_secs(10) {
                // Heartbeat timed out
                println!("WebSocket Client heartbeat failed, disconnecting!");
                // Notify chat server
                act.addr.do_send(Disconnect { id: act.id });
                // Stop actor
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called when the actor is started
    fn started(&mut self, ctx: &mut Self::Context) {
        // Start heartbeat process
        self.hb(ctx);

        // Register self in chat server
        let addr = ctx.address();

        // Send Connect message without waiting for a response
        self.addr.do_send(Connect {
            id: self.id,
            addr: addr.recipient(),
        });
    }

    /// Method is called when the actor is stopped
    fn stopped(&mut self, _: &mut Self::Context) {
        // Unregister from chat server
        self.addr.do_send(Disconnect { id: self.id });
    }
}

/// Handle messages received from chat server
impl Handler<ChatMessage> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// Handle incoming WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                // Convert ByteString to String
                let message = text.to_string();
                // Broadcast message to other clients
                self.addr.do_send(ChatMessage(message));
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Err(e) => {
                println!("WebSocket error: {}", e);
                ctx.stop();
            }
            Ok(_) => {
                // Handle any other messages (e.g., ws::Message::Nop)
                // You can log or ignore them
            }
        }
    }
}

/// WebSocket route handler
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    let session = ChatSession::new(srv.get_ref().clone());
    ws::start(session, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start chat server actor
    let server = ChatServer::new().start();

    println!("Server running on ws://127.0.0.1:8080/ws/");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .route("/ws/", web::get().to(chat_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
