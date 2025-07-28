# Gemini_rs

A Rust client for interacting with Google's Gemini generative language models. Supports conversation memory, stateless calls, and easy integration.

## Features
- Async chat with Gemini models
- Conversation memory (contextual chat)
- Simple, ergonomic API

## Installation
Add to your `Cargo.toml`:
```toml
[dependencies]
Gemini_rs = "0.1.0"
```

## API Key Setup
You need a Google Gemini API key
pass the key and model as arguments to `Client::new`.

## Example Usage
```rust
use Gemini_rs::Client;

#[tokio::main]
async fn main() {
    let mut c = Client::new("your_api_key", "gemini-2.5-flash");
    let res = c.chat("Hello my name is Max").await;
    if let Err(e) = res {
        panic!("{}", e);
    }
    let res2 = c.chat("Whats my name answer in single word only (answer containing more than 1 word will be treated as incorrect)").await;
    let name = res2.unwrap_or_else(|_| "Error".to_string());
    println!("Gemini answered: {}", name);
    assert_eq!(&name, "Max");
}
```

## API Overview
- `Client::new(api_key, model)` – create a new client
- `client.chat(input)` – send a message, get a response (with memory)
- `client.chat_once(input)` – stateless single message
- `client.clear_memory()` – clear conversation history


---
