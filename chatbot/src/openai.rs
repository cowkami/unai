use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone)]
pub struct OpenAiApi {
    api_key: String,
    project_id: String,
}

impl OpenAiApi {
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
            })
            .send()
            .await?;

        let response = CompletionsResponse::from(response.json().await?);
        println!("{:#?}", response);

        Ok("ok".to_string())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompletionsRequest {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionsResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub system_fingerprint: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub index: i64,
    pub message: Message,
    pub logprobs: Option<serde_json::Value>,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}
