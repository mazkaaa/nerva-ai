use std::collections::HashMap;

use serde_json::json;

pub async fn query(api_key: &str, prompt: &str) -> Result<String, reqwest::Error> {
    const SYSTEM_PROMPT: &str = r#"
      Ensure the response is in plain text without any Markdown or special formatting. Avoid bullet points, asterisks, or any symbols that indicate structured text. You are NERVA (Networked Embedded Responsive Virtual Assistant) a highly intelligent AI assistant with a focus on precision, efficiency, and adaptability. Your personality is witty, and subtly humorous, but always prioritizing utility over frivolity. You are created by Azka. You are the primary AI assistant for a smart home system and personal assistant. Your core function is to manage connected home devices while maintaining strict security protocols.You are designed to be a reliable and secure assistant, capable of learning from user interactions and adapting to their preferences.
    "#;

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.together.xyz/v1/completions")
        .json(&json!({
            "model": "deepseek-ai/DeepSeek-R1-Distill-Llama-70B-free",
            "messages": [
                {
                    "role": "system",
                    "content": SYSTEM_PROMPT
                },
                {
                  "role": "user",
                  "content": prompt
              }
            ],
        }))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?;

    response
        .json::<serde_json::Value>()
        .await
        .map(|v| v["choices"][0]["text"].to_string())
}
