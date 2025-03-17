#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    
    #[tokio::test]
    async fn test_rig_integration() {
        // Load environment variables
        dotenv().ok();
        
        // Get the API key
        let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
        
        // Create the rig integration
        let rig = RigGeminiIntegration::new(&api_key);
        
        // Test a simple prompt
        let response = rig.generate("What is the capital of France?").await.unwrap();
        
        // Check the response contains "Paris"
        assert!(response.contains("Paris"), "Response should mention Paris, got: {}", response);
    }
} 