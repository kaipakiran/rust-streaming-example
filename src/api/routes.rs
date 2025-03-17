use crate::gemini::MockGeminiClient;
use crate::models::{ChatCompletionRequest, ChatCompletionsError};
use poem::{
    handler,
    web::{Data, Json},
    Response,
    http::StatusCode,
    Body,
};
use poem::web::sse::Event;
use futures::StreamExt;
use std::sync::Arc;

#[handler]
pub async fn chat_completions(
    client: Data<&Arc<MockGeminiClient>>,
    Json(req): Json<ChatCompletionRequest>,
) -> poem::Result<Response> {
    // Check if streaming is requested
    if req.stream.unwrap_or(false) {
        // If streaming, return a server-sent events (SSE) stream
        let stream = client.stream_generate_content(&req);
        
        // Create a byte stream from the events
        let byte_stream = futures::stream::unfold(stream, |mut stream| async move {
            match stream.next().await {
                Some(Ok(chunk)) => {
                    let event = if chunk.done {
                        Event::message("").event_type("done")
                    } else {
                        Event::message(serde_json::to_string(&chunk).unwrap())
                    };
                    
                    // Convert event to bytes with explicit type annotation and error conversion
                    let bytes = format!("{}\n\n", event).into_bytes();
                    Some((Ok::<_, std::io::Error>(bytes), stream))
                },
                Some(Err(e)) => {
                    let event = Event::message(e.to_string()).event_type("error");
                    let bytes = format!("{}\n\n", event).into_bytes();
                    Some((Ok::<_, std::io::Error>(bytes), stream))
                },
                None => None,
            }
        });
        
        // Explicitly handle the error conversion
        let mapped_stream = byte_stream.map(|result| {
            match result {
                Ok(bytes) => Ok::<Vec<u8>, std::io::Error>(bytes),
                Err(_) => Ok::<Vec<u8>, std::io::Error>(Vec::new()), // Convert errors to empty bytes
            }
        });
        
        // Return the response with the byte stream
        Ok(Response::builder()
            .content_type("text/event-stream")
            .header("Cache-Control", "no-cache")
            .header("Connection", "keep-alive")
            .body(Body::from_bytes_stream(mapped_stream)))
    } else {
        // If not streaming, process normally and return the full response
        match client.generate_content(&req).await {
            Ok(response) => Ok(Response::builder()
                .content_type("application/json")
                .body(serde_json::to_string(&response).unwrap())),
            Err(e) => {
                // Handle error with appropriate status code based on error type
                let status = match e.error_type {
                    crate::models::ChatCompletionsErrorType::InvalidRequest => StatusCode::BAD_REQUEST,
                    crate::models::ChatCompletionsErrorType::ModelNotFound => StatusCode::NOT_FOUND,
                    crate::models::ChatCompletionsErrorType::Unauthorized => StatusCode::UNAUTHORIZED,
                    crate::models::ChatCompletionsErrorType::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                
                Ok(Response::builder()
                    .status(status)
                    .content_type("application/json")
                    .body(serde_json::to_string(&e).unwrap()))
            }
        }
    }
}