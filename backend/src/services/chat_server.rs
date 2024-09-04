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
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Context<Self>) {
        let ai_response = self.ai_model.generate_response(msg.msg.clone());
        if let Ok(response) = ai_response {
            let ai_message = Message::new(response, "AI".to_string());
            if let Some(addr) = self.sessions.get(&msg.id) {
                addr.do_send(ai_message);
            }
        }
    }
}
