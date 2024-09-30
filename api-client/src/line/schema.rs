use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReplyMessage {
    pub reply_token: String,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PushMessage {
    pub to: String,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WebhookEvent {
    pub destination: String,
    pub events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub r#type: EventType,
    pub message: Message,
    pub timestamp: i64,
    pub source: Source,
    pub reply_token: String,
    pub mode: String,
    pub webhook_event_id: String,
    pub delivery_context: DeliveryContext,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum EventType {
    Message,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Message {
    Text(TextMessage),
    Image(ImageMessage),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextMessage {
    pub id: Option<String>,
    pub r#type: String,
    pub text: String,
    pub quote_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImageMessage {
    pub r#type: String,
    pub original_content_url: String,
    pub preview_image_url: String,
}

impl Message {
    pub fn text(text: String, quote_token: Option<String>) -> Self {
        Self::Text(TextMessage {
            id: None,
            r#type: "text".to_string(),
            text,
            quote_token,
        })
    }

    pub fn image(original_content_url: String, preview_image_url: String) -> Self {
        Self::Image(ImageMessage {
            r#type: "image".to_string(),
            original_content_url,
            preview_image_url,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub r#type: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryContext {
    pub is_redelivery: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadingStart {
    pub chat_id: String,
    pub loading_seconds: i64,
}
