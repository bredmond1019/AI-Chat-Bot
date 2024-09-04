use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage, ChatMessageResponseStream},
    Ollama,
};
use tokio_stream::StreamExt;

pub struct AIModel {
    ollama: Ollama,
}

impl AIModel {
    pub fn new() -> Self {
        Self {
            ollama: Ollama::new_default_with_history(30),
        }
    }

    pub async fn generate_response(
        &self,
        input: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut stream: ChatMessageResponseStream = self
            .ollama
            .send_chat_messages_with_history_stream(
                ChatMessageRequest::new(
                    "llama3.1:latest".to_string(),
                    vec![ChatMessage::user(input)],
                ),
                "user".to_string(),
            )
            .await?;

        let mut response = String::new();
        while let Some(Ok(res)) = stream.next().await {
            if let Some(assistant_message) = res.message {
                response += assistant_message.content.as_str();
            }
        }

        Ok(response)
    }
}
