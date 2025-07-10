use crate::models::{GeminiRequest, GenerationConfig, ThinkingConfig, GeminiContent, GeminiResponse};
use crate::client::ClientConfig;
use reqwest::Client as HttpClient;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

pub async fn call_api_with_config(
    config: &ClientConfig,
    messages: Vec<GeminiContent>,
) -> Result<GeminiResponse, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        config.model, config.api_key
    );

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = HttpClient::builder().default_headers(headers).build()?;

    // Create the request payload
    let request_payload = GeminiRequest {
        contents: messages,
        generation_config: GenerationConfig {
            thinking_config: ThinkingConfig {
                thinking_budget: 0,
            },
            response_mime_type: "text/plain".to_string(),
        },
    };

    // Make the API call
    let response = client
        .post(&url)
        .json(&request_payload)
        .send()
        .await?;

    // Check if the response is successful
    if !response.status().is_success() {
        let status_code = response.status().as_u16();
        let error_text = response.text().await?;
        return Err(format!("API call failed with status {}: {}", status_code, error_text).into());
    }

    // Parse the response
    let response_text = response.text().await?;
    let gemini_response: GeminiResponse = serde_json::from_str(&response_text)?;
    Ok(gemini_response)
}

// Keep the original function for backward compatibility
pub async fn call_api(messages: Vec<GeminiContent>) -> Result<GeminiResponse, Box<dyn std::error::Error + Send + Sync>> {
    use dotenv::dotenv;
    use std::env;

    dotenv().ok();
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");
    let model = env::var("GEMINI_MODEL").expect("GEMINI_MODEL not set");

    let config = ClientConfig {
        api_key,
        model,
        max_memory_size: 50,
    };

    call_api_with_config(&config, messages).await
}