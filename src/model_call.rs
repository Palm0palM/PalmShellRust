use std::env;
use dotenvy::dotenv;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde_json::{json, Value};

use crate::error::ShellError;

pub struct Config {
    api_url: String,
    api_key: String,
    model_name: String,
}
impl Config {
    pub fn new(url: String, key: String, name: String) -> Self{
        Self{
            api_url: url,
            api_key: key,
            model_name: name,
        }
    }
}

pub async fn llm_call(message: String) -> Result<String, ShellError> {
    dotenv().ok();

    let client = Client::new();

    let config = Config::new(
        env::var("LLM_API_URL").expect("psh: LLM_API_URL not set"),
        env::var("LLM_API_KEY").expect("psh: LLM_API_KEY not set"),
        env::var("LLM_MODEL_NAME").expect("psh: LLM_MODEL_NAME not set"),
    );

    let payload = json!({
        "model": config.model_name,
        "messages": [
            {
                "role": "user",
                "content": message
            }
        ],
    });

    let res = client.post(&config.api_url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", config.api_key))
        .json(&payload)
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let json_resp: Value = response.json().await.unwrap();
                if let Some(content) = json_resp["choices"][0]["message"]["content"].as_str() {
                    Ok(format!("\nAI: {}\n", content))
                } else {
                    Err(ShellError::LLMError("Unknown API Error".to_string()))
                }
            } else {
                Err(ShellError::LLMError(format!("API Error: {:?}", response.text().await)))
            }
        }
        Err(e) => Err(ShellError::LLMError(format!("Connection Error: {}", e))),
    }
}