use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    pub content: String,
    pub sender: String,
    pub timestamp: DateTime<Utc>,
}

impl Message {
    pub fn new(content: String, sender: String) -> Self {
        Self {
            content,
            sender,
            timestamp: Utc::now(),
        }
    }
}
