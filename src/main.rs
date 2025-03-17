mod api;
mod gemini;
mod models;
// mod rig;  // Comment this out until we have the rig module properly set up

use api::routes;
use dotenv::dotenv;
use gemini::MockGeminiClient;
use poem::{
    post, 
    middleware::Cors,
    EndpointExt, Route, Server,
    listener::TcpListener,
};
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv().ok();
    
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Initialize the mock Gemini client
    let gemini_client = Arc::new(MockGeminiClient::new());

    // Configure API routes
    let app = Route::new()
        .at("/v1/chat/completions", post(routes::chat_completions))
        .with(Cors::new())
        .data(gemini_client);

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    Server::new(TcpListener::bind(addr))
        .run(app)
        .await
        .unwrap();

    // Add this to your main.rs to demonstrate direct rig-core usage

    /* 
    // Example of using rig directly
    async fn example_direct_rig_usage() -> Result<(), Box<dyn std::error::Error>> {
        // Load API key from environment
        let api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
        
        // Create rig gemini client
        let client = rig::providers::gemini::Client::new(&api_key);
        
        // Create a completion model for gemini-pro
        let model = client.completion_model("gemini-2.0-flash");
        
        // Define a conversation
        let messages = vec![
            ("user", "Hello, who are you?"),
        ];
        
        // Generate a completion
        let response = model.complete_chat(&messages).await?;
        println!("Response: {}", response);
        
        Ok(())
    }
    */
}
