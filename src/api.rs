use reqwest::blocking::Client;
use reqwest::header;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub stream: bool,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub message: Message,
}

pub struct ApiClient {
    client: Client,
    config: crate::config::Config,
}

impl ApiClient {
    pub fn new(config: crate::config::Config) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&format!("Bearer {}", config.api_key))
                .expect("Invalid API key"),
        );
        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/json"),
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create HTTP client");

        ApiClient { client, config }
    }

    pub fn send_message(&self, messages: Vec<Message>) -> Result<String, Box<dyn Error>> {
        let request = ChatRequest {
            model: "deepseek-ai/DeepSeek-R1-Distill-Llama-70B-free".to_string(),
            messages,
            temperature: 0.8,
            stream: false,
        };

        let response = self
            .client
            .post(&self.config.api_url)
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            return Err(format!("API request failed: {}", response.status()).into());
        }

        let response: ChatResponse = response.json()?;
        if let Some(choice) = response.choices.get(0) {
            Ok(choice.message.content.clone())
        } else {
            Err("No response from AI".into())
        }
    }
}
