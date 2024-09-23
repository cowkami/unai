use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionsRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: Option<f64>,
    pub response_format: Option<ResponseFormat>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseFormat {
    pub r#type: String,
    pub json_schema: JsonSchema,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonSchema {
    pub name: String,
    pub schema: serde_json::Value,
    pub strict: bool,
}

impl ResponseFormat {
    pub fn new(schema_name: String, json_schema: serde_json::Value) -> Self {
        ResponseFormat {
            r#type: "json_schema".to_string(),
            json_schema: JsonSchema {
                name: schema_name,
                schema: json_schema,
                strict: true,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDemand {
    pub user_demand: String,
}
