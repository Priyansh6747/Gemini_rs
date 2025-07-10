use crate::models::{GeminiContent, GeminiResponse};
use crate::api::call_api_with_config;
use std::collections::VecDeque;

/// Configuration for the Gemini client
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub api_key: String,
    pub model: String,
    pub max_memory_size: usize,
}

/// Gemini AI client with conversation memory
pub struct Client {
    config: ClientConfig,
    memory: VecDeque<GeminiContent>,
}

impl Client {
    /// Create a new client instance
    pub fn new(api_key: &str, model: &str) -> Self {
        Self {
            config: ClientConfig {
                api_key: api_key.to_string(),
                model: model.to_string(),
                max_memory_size: 50, // Default max conversation history
            },
            memory: VecDeque::new(),
        }
    }

    /// Create a new client with custom configuration
    pub fn with_config(config: ClientConfig) -> Self {
        Self {
            config,
            memory: VecDeque::new(),
        }
    }

    /// Send a chat message and get response
    pub async fn chat(&mut self, input: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Add user message to memory
        let user_message = GeminiContent::new("user".to_string(), input.to_string());
        self.add_to_memory(user_message.clone());

        // Prepare messages for API call (include conversation history)
        let messages: Vec<GeminiContent> = self.memory.iter().cloned().collect();

        // Make API call with current config
        let response = call_api_with_config(&self.config, messages).await?;

        // Extract response text
        let response_text = response.get_string()
            .ok_or("No response text found")?;

        // Add assistant response to memory
        let assistant_message = GeminiContent::new("model".to_string(), response_text.clone());
        self.add_to_memory(assistant_message);

        Ok(response_text)
    }

    /// Send a single message without memory (stateless)
    pub async fn chat_once(&self, input: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let user_message = GeminiContent::new("user".to_string(), input.to_string());
        let messages = vec![user_message];

        let response = call_api_with_config(&self.config, messages).await?;
        let response_text = response.get_string()
            .ok_or("No response text found")?;

        Ok(response_text)
    }

    /// Clear conversation memory
    pub fn clear_memory(&mut self) {
        self.memory.clear();
    }

    /// Get current memory size
    pub fn memory_size(&self) -> usize {
        self.memory.len()
    }

    /// Get conversation history
    pub fn get_history(&self) -> Vec<GeminiContent> {
        self.memory.iter().cloned().collect()
    }

    /// Set maximum memory size
    pub fn set_max_memory_size(&mut self, size: usize) {
        self.config.max_memory_size = size;
        self.trim_memory();
    }

    /// Add message to memory with size management
    fn add_to_memory(&mut self, message: GeminiContent) {
        self.memory.push_back(message);
        self.trim_memory();
    }

    /// Trim memory to max size (removes oldest messages)
    fn trim_memory(&mut self) {
        while self.memory.len() > self.config.max_memory_size {
            self.memory.pop_front();
        }
    }
}