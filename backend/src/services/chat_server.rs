use crate::models::message::Message;
use crate::services::ai_model::AIModel;
use actix::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

pub struct ChatServer {
    sessions: HashMap<Uuid, Recipient<Message>>,
    ai_model: AIModel,
}

impl ChatServer {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            ai_model: AIModel::new(),
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: Uuid,
    pub msg: String,
}

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

impl Handler<ClientMessage> for ChatServer {
    type Result = ResponseFuture<()>;

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) -> Self::Result {
        let mut ai_model = self.ai_model.clone();
        let sessions = self.sessions.clone();
        let id = msg.id;

        Box::pin(async move {
            if let Ok(response) = ai_model.generate_response(msg.msg).await {
                let ai_message = Message::new(response, "AI".to_string());
                if let Some(addr) = sessions.get(&id) {
                    addr.do_send(ai_message);
                }
            }
        })
    }
}
