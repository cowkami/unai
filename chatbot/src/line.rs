use serde::{Deserialize, Serialize};

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
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: String,
    pub r#type: String,
    pub text: String,
    pub quote_token: String,
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
