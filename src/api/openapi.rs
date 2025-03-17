use crate::models::{ChatCompletionRequest, ChatCompletionResponse};
use utoipa::OpenApi;

// Simplified OpenAPI definition
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Chat Completions API",
        version = "1.0.0",
        description = "Mock LLM chat API with streaming support"
    ),
    paths(
        chat_completions
    ),
    components(
        schemas(ChatCompletionRequest, ChatCompletionResponse)
    ),
    tags(
        (name = "chat", description = "Chat completion endpoints")
    )
)]
pub struct ApiDoc;

#[utoipa::path(
    post,
    path = "/v1/chat/completions",
    request_body = ChatCompletionRequest,
    responses(
        (status = 200, description = "Chat completion generated successfully", body = ChatCompletionResponse)
    ),
    tag = "chat"
)]
fn chat_completions() {}

// We can add proper path definitions later 