use crate::models::message::Message;
use crate::services::ai_model::AIModel;
use actix::prelude::*;
use log::{error, info};
use serde::Deserialize;
use std::collections::HashMap;

use super::chat_session::SessionId;

pub struct ChatServer {
    sessions: HashMap<SessionId, Recipient<Message>>,
    ai_model: AIModel,
}

impl ChatServer {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            ai_model: AIModel::new(),
        }
    }

    fn send_message_to_sessions(&self, message: Message) {
        for (id, addr) in &self.sessions {
            addr.do_send(message.clone());
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
    pub id: SessionId,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: SessionId,
}

#[derive(Message, Deserialize)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: SessionId,
    pub msg: String,
}

impl Handler<Connect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        info!("ChatSession connected: {:?}", msg.id);
        self.sessions.insert(msg.id, msg.addr);
        info!("Total active sessions: {}", self.sessions.len());
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        info!("ChatSession disconnected: {:?}", msg.id);
        self.sessions.remove(&msg.id);
        info!("Total active sessions: {}", self.sessions.len());
    }
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ResponseFuture<()>;

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) -> Self::Result {
        info!("Received message from session {:?}: {}", msg.id, msg.msg);
        let mut ai_model = self.ai_model.clone();
        let sessions = self.sessions.clone();
        let id = msg.id;

        Box::pin(async move {
            info!("Generating AI response for session {:?}", id);
            match ai_model.generate_response(msg.msg).await {
                Ok(response) => {
                    info!("AI response generated for session {:?}", id);
                    let ai_message = Message::new(response, "AI".to_string());
                    if let Some(addr) = sessions.get(&id) {
                        info!("Sending AI response to session {:?}", id);
                        addr.do_send(ai_message);
                    } else {
                        error!("Session {:?} not found", id);
                    }
                }
                Err(e) => {
                    error!("Failed to generate AI response for session {:?}: {}", id, e);
                    if let Some(addr) = sessions.get(&id) {
                        let error_message = Message::new(
                            "Sorry, I couldn't process your request. Please try again.".to_string(),
                            "AI".to_string(),
                        );
                        info!("Sending error message to session {:?}", id);
                        addr.do_send(error_message);
                    }
                }
            }
        })
    }
}
