use actix::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Message, Serialize, Deserialize, Debug, Clone)]
#[rtype(result = "()")]
pub struct Message {
    pub content: String,
    pub sender: String,
}

impl Message {
    pub fn new(content: String, sender: String) -> Self {
        Self { content, sender }
    }
}
