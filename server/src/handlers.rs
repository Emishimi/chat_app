use actix::{Actor, StreamHandler};
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use actix::ActorContext;

/// Define a struct for your WebSocket session.
/// Each instance of `ChatSession` will be created for each connected user.
pub struct ChatSession;

impl ChatSession {
    pub fn new() -> Self {
        ChatSession {}
    }
}

/// Implement Actor trait for `ChatSession`.
/// This will set the context for this actor as `WebsocketContext`.
impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;
}

/// Implement the WebSocket message handling.
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),  // Echoes back the text
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
            }
            _ => (),
        }
    }
}

/// Handler for WebSocket requests
pub async fn chat_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(ChatSession::new(), &req, stream)
}

/// Simple HTTP handler function
pub async fn greet() -> impl actix_web::Responder {
    "Hello, world!"
}
