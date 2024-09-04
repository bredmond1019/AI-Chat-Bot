use futures::stream::Stream;
use log::{error, info};
use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage, ChatMessageResponseStream},
    Ollama,
};
use std::error::Error as StdError;
use std::fmt;
use std::pin::Pin;
use tokio_stream::StreamExt;

#[derive(Clone)]
pub struct AIModel {
    ollama: Ollama,
}

impl AIModel {
    pub fn new() -> Self {
        info!("Initializing new AIModel");
        Self {
            ollama: Ollama::new_default_with_history(30),
        }
    }

    pub async fn generate_response(
        &mut self,
        input: String,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<String, Box<dyn std::error::Error>>> + Send>>,
        Box<dyn std::error::Error>,
    > {
        info!("Generating AI response for input: {}", input);
        let stream: ChatMessageResponseStream = self
            .ollama
            .send_chat_messages_with_history_stream(
                ChatMessageRequest::new(
                    "llama3.1:latest".to_string(),
                    vec![ChatMessage::user(input.clone())],
                ),
                "user".to_string(),
            )
            .await
            .map_err(|e| {
                error!("Failed to send chat message: {}", e);
                AIModelError::RequestError(e.to_string())
            })?;

        info!("Successfully initiated chat message stream");

        Ok(Box::pin(stream.map(|res| match res {
            Ok(chunk) => {
                if let Some(assistant_message) = chunk.message {
                    info!("Received chunk of AI response");
                    Ok(assistant_message.content)
                } else {
                    Ok(String::new())
                }
            }
            Err(e) => {
                error!("Error while streaming response: {:?}", e);
                Err(Box::new(AIModelError::StreamingError(format!("{:?}", e)))
                    as Box<dyn std::error::Error>)
            }
        })))
    }
}

#[derive(Debug)]
enum AIModelError {
    RequestError(String),
    StreamingError(String),
    EmptyResponse,
}

impl fmt::Display for AIModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AIModelError::RequestError(e) => write!(f, "Request error: {}", e),
            AIModelError::StreamingError(e) => write!(f, "Streaming error: {}", e),
            AIModelError::EmptyResponse => write!(f, "Empty response generated"),
        }
    }
}

impl StdError for AIModelError {}
