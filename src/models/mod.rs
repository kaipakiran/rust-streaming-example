use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ChatCompletionRequest {
    pub messages: Vec<Message>,
    pub model: String,
    pub stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, ToSchema)]
pub struct ChatCompletionResponse {
    pub answer: String,
    pub title: Option<String>,
    pub pills: Option<Vec<String>>,
    pub sources: Option<Vec<Source>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Source {
    pub title: String,
    pub url: String,
    pub publication_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StreamChunk {
    pub answer: String,
    pub done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum ChatCompletionsErrorType {
    InvalidRequest,
    ModelNotFound,
    InternalError,
    RateLimitExceeded,
    Unauthorized,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ChatCompletionsError {
    pub error_type: ChatCompletionsErrorType,
    pub message: String,
    pub request_id: Option<String>,
}

impl fmt::Display for ChatCompletionsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.error_type, self.message)
    }
}

impl std::error::Error for ChatCompletionsError {}

// Helper methods for creating common errors
impl ChatCompletionsError {
    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self {
            error_type: ChatCompletionsErrorType::InvalidRequest,
            message: message.into(),
            request_id: None,
        }
    }

    pub fn internal_error(message: impl Into<String>) -> Self {
        Self {
            error_type: ChatCompletionsErrorType::InternalError,
            message: message.into(),
            request_id: None,
        }
    }
    
    // You can add more helper methods as needed
} 