pub mod schema;

use schema::{CompletionsRequest, CompletionsResponse, Message};
use std::env;

#[derive(Clone)]
pub struct Gpt {
    api_key: String,
    project_id: String,
}

impl Gpt {
    pub fn new() -> Result<Self, &'static str> {
        let api_key =
            env::var("OPENAI_API_KEY").expect("Please set the OPENAI_API_KEY environment variable");
        let project_id = env::var("OPENAI_PROJECT_ID")
            .expect("Please set the OPENAI_PROJECT_ID environment variable");
        Ok(Self {
            api_key,
            project_id,
        })
    }

    pub async fn send_chat(&self, chat: &str) -> Result<String, reqwest::Error> {
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&CompletionsRequest {
                model: "gpt-4o".to_string(),
                messages: vec![Message {
                    role: "user".to_string(),
                    content: chat.to_string(),
                }],
                temperature: Some(0.7),
            })
            .send()
            .await?;

        let response: CompletionsResponse = response.json().await?;

        let text = response.choices[0].message.content.clone();
        Ok(text)
    }
}
