use crate::models::message::Message;
use crate::services::chat_server::{ChatServer, ClientMessage, Connect, Disconnect};
use actix::{Actor, ActorContext, AsyncContext, Handler, StreamHandler};
use actix_web_actors::ws;
use uuid::Uuid;

pub struct ChatSession {
    id: Uuid,
    addr: actix::Addr<ChatServer>,
}

impl ChatSession {
    pub fn new(addr: actix::Addr<ChatServer>) -> Self {
        Self {
            id: Uuid::new_v4(),
            addr,
        }
    }
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.addr
            .send(Connect {
                addr: addr.recipient(),
                id: self.id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                if let Err(e) = res {
                    eprintln!("Error connecting: {}", e);
                    ctx.stop();
                }
                actix::fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
        self.addr.do_send(Disconnect { id: self.id });
        actix::Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let msg = text.trim();
                if !msg.is_empty() {
                    self.addr.do_send(ClientMessage {
                        id: self.id,
                        msg: msg.to_string(),
                    });
                }
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl Handler<Message> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}
