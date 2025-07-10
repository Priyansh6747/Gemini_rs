/// Gemini_rs: Async Rust client for Google Gemini generative language models.
///
/// # Features
/// - Async chat with Gemini models
/// - Conversation memory (contextual chat)
/// - Simple, ergonomic API
///
/// # Example
/// ```rust
/// use Gemini_rs::Client;
/// #[tokio::main]
/// async fn main() {
///     let mut c = Client::new("your_api_key", "gemini-2.5-flash");
///     let res = c.chat("Hello my name is Max").await;
///     if let Err(e) = res {
///         panic!("{}", e);
///     }
///     let res2 = c.chat("Whats my name answer in single word only (answer containing more than 1 word will be treated as incorrect)").await;
///     let name = res2.unwrap_or_else(|_| "Error".to_string());
///     println!("Gemini answered: {}", name);
///     assert_eq!(&name, "Max");
/// }
/// ```
///
/// # API
/// ## Main Structs
/// - [`Client`]: main entry point for chat and memory
/// - [`GeminiContent`], [`GeminiPart`], [`GeminiRequest`], [`GeminiResponse`], [`Candidate`], [`Content`], [`Part`], [`SafetyRating`]
///
/// ## Client Methods
/// - `Client::new(api_key, model)` – create a new client
/// - `Client::with_config(config)` – create with custom config
/// - `client.chat(input)` – send a message, get a response (async, with memory)
/// - `client.chat_once(input)` – stateless single message (async)
/// - `client.clear_memory()` – clear conversation history
/// - `client.memory_size()` – get memory size
/// - `client.get_history()` – get conversation history
/// - `client.set_max_memory_size(size)` – set max memory size
///
/// ## Model/Response Utilities
/// - `GeminiContent::new(role, content)` – create a message
/// - `GeminiResponse::from_json(json_str)` – parse from JSON
/// - `GeminiResponse::from_value(value)` – parse from serde_json::Value
/// - `GeminiResponse::extract_text()` – get first generated text
/// - `GeminiResponse::extract_all_texts()` – get all generated texts
/// - `GeminiResponse::get_string()` – get cleaned response string
///
/// ## Advanced API
/// - `call_api_with_config(config, messages)` – low-level async API call
/// - `call_api(messages)` – async API call using env vars
// lib.rs
mod models;
mod api;
mod client;

use std::error::Error;
pub use client::Client;
pub use models::*;

#[tokio::test]
async fn trying() {
    ///you can use any available model of ur liking
    let mut c = Client::new("key", "gemini-2.5-flash");
    let res = c.chat("Hello my name is Max").await;
    if res.is_err() {
    panic!("{}", res.err().unwrap());}
    let res2 = c.chat("Whats my name answer in single word only (answer containing more than 1 word will be treated as incorrect)").await;
    let name = res2.unwrap_or_else(|_| "Error".to_string());
    dbg!(&name);
    assert_eq!(&name, "Max");
}