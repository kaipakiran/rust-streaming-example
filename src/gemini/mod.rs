use crate::models::{ChatCompletionRequest, ChatCompletionResponse, ChatCompletionsError, StreamChunk, Source};
use async_stream::try_stream;
use futures::Stream;
use serde_json::json;
use std::pin::Pin;
use std::time::Duration;

pub struct MockGeminiClient;

impl MockGeminiClient {
    pub fn new() -> Self {
        Self
    }

    pub async fn generate_content(
        &self,
        request: &ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, ChatCompletionsError> {
        // Extract the user message (taking the last user message)
        let user_message = request.messages.iter()
            .filter(|msg| msg.role == "user")
            .last()
            .map(|msg| msg.content.clone())
            .unwrap_or_else(|| {
                // Return an error if no user message is found
                return "No user message found".to_string();
            });
        
        // Simply echo back the user's message
        Ok(ChatCompletionResponse {
            answer: format!("Echo: {}", user_message),
            title: Some("Mocked Response".to_string()),
            pills: Some(vec!["Mock".to_string(), "Echo".to_string()]),
            sources: None,
        })
    }

    pub fn stream_generate_content(
        &self,
        request: &ChatCompletionRequest,
    ) -> Pin<Box<dyn Stream<Item = Result<StreamChunk, ChatCompletionsError>> + Send>> {
        // Extract the user message (taking the last user message)
        let user_message = request.messages.iter()
            .filter(|msg| msg.role == "user")
            .last()
            .map(|msg| msg.content.clone())
            .unwrap_or_else(|| "No user message found".to_string());
        
        Box::pin(try_stream! {
            // Split the message into words for streaming simulation
            let words: Vec<&str> = user_message.split_whitespace().collect();
            
            // Stream each word with a small delay to simulate streaming
            for (i, word) in words.iter().enumerate() {
                // Small delay to simulate processing time
                tokio::time::sleep(Duration::from_millis(100)).await;
                
                let response_text = if i == 0 {
                    format!("Echo: {}", word)
                } else {
                    format!(" {}", word)
                };
                
                // Create a partial response object
                let response_obj = json!({
                    "answer": response_text,
                    "title": "Mocked Response",
                    "pills": ["Mock", "Echo"],
                    "sources": []
                });
                
                yield StreamChunk {
                    answer: response_obj.to_string(),
                    done: false,
                };
            }
            
            // Final chunk with complete response
            let final_response = json!({
                "answer": format!("Echo: {}", user_message),
                "title": "Mocked Response",
                "pills": ["Mock", "Echo"],
                "sources": [
                    {
                        "title": "Mock Source",
                        "url": "https://example.com",
                        "publication_date": "2023-01-01"
                    }
                ]
            });
            
            yield StreamChunk {
                answer: final_response.to_string(),
                done: true,
            };
        })
    }
}

// Add Clone implementation for our mock
impl Clone for MockGeminiClient {
    fn clone(&self) -> Self {
        Self
    }
} 