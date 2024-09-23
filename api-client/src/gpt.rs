pub mod schema;

use domain::user::UserDemand;
use schema::*;
use serde_json::json;
use std::env;

#[derive(Clone)]
pub struct Gpt {
    api_key: String,
}

impl Gpt {
    pub fn new() -> Result<Self, &'static str> {
        let api_key =
            env::var("OPENAI_API_KEY").expect("Please set the OPENAI_API_KEY environment variable");
        Ok(Self { api_key })
    }

    pub async fn completions(
        &self,
        request: CompletionsRequest,
    ) -> Result<CompletionsResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        response.json().await
    }

    pub async fn generate_image(&self, prompt: String) -> Result<Vec<String>, reqwest::Error> {
        let request = GenerateImageRequest {
            model: "dall-e-2".to_string(),
            prompt,
            n: 1,
            size: "256x256".to_string(),
            response_format: Some(GenImageResponseFormat::B64Json),
        };
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.openai.com/v1/images/generations")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        let response: GenerateImageResponse = response.json().await?;
        let images: Vec<String> = response
            .data
            .into_iter()
            .map(|image| image.b64_json)
            .collect();

        Ok(images)
    }

    pub async fn chat(&self, chat: String) -> Result<String, reqwest::Error> {
        let request = CompletionsRequest {
            model: "gpt-4o".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: chat,
            }],
            temperature: Some(0.7),
            response_format: None,
        };
        let response = self.completions(request).await?;

        let text = response.choices[0].message.content.clone();
        Ok(text)
    }

    pub async fn detect_demand(&self, chat: String) -> Result<UserDemand, reqwest::Error> {
        let response_format = ResponseFormat::new(
            "user_demand".to_string(),
            json!({
                "type": "object",
                "properties": {
                    "user_demand": {
                        "type": "string",
                        "enum": ["Chat", "CreateImage"],
                    }
                },
                "required": ["user_demand"],
                "additionalProperties": false,
            }),
        );
        let request = CompletionsRequest {
            model: "gpt-4o-mini".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "You are an expert at detecting user demand. \
                        Choose one appropriate label as the user demands from the following options:\n\
                        - Chat\n\
                        - CreateImage"
                        .to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: chat,
                },
            ],
            temperature: Some(0.0),
            response_format: Some(response_format),
        };

        let response = self.completions(request).await?;
        let content = response.choices[0].message.content.clone();
        let user_demand: schema::UserDemand = serde_json::from_str(&content).unwrap();
        let user_demand = UserDemand::try_from(user_demand.user_demand).unwrap();

        Ok(user_demand)
    }
}
