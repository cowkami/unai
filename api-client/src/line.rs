pub mod schema;

use domain::{
    message::{Actor, Message},
    user::User,
};
use schema::{
    Event, EventType, LoadingStart, Message as LineMessage, PushMessage, ReplyMessage, TextMessage,
    WebhookEvent,
};

#[derive(Clone)]
pub struct Line {
    channel_access_token: String,
    bot_user_id: String,
}

impl Line {
    pub fn new() -> Result<Self, &'static str> {
        let channel_access_token = std::env::var("LINE_CHANNEL_ACCESS_TOKEN")
            .expect("Please set the LINE_CHANNEL_ACCESS_TOKEN environment variable");
        let bot_user_id = std::env::var("LINE_BOT_USER_ID")
            .expect("Please set the LINE_BOT_USER_ID environment variable");

        Ok(Self {
            channel_access_token,
            bot_user_id,
        })
    }

    pub fn get_user_message(&self, payload: WebhookEvent) -> Result<Message, &'static str> {
        let message_event = payload
            .events
            .into_iter()
            .filter(|event| matches!(event.r#type, EventType::Message))
            .next()
            .expect("No message event found");

        // extract message from event
        let message = self.extract_message(message_event);

        Ok(message)
    }

    fn extract_message(&self, event: Event) -> Message {
        match event.message {
            schema::Message::Text(TextMessage { text, .. }) => Message {
                from: Actor::User(User {
                    id: event.source.user_id,
                }),
                to: Actor::Bot,
                text,
                reply_token: Some(event.reply_token),
            },
            schema::Message::Image(_) => unimplemented!(),
        }
    }

    pub async fn show_loading(&self) -> Result<(), &'static str> {
        let client = reqwest::Client::new();
        client
            .post("https://api.line.me/v2/bot/chat/loading/start")
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", self.channel_access_token),
            )
            .json(&LoadingStart {
                chat_id: self.bot_user_id.clone(),
                loading_seconds: 60, // maximum 60 seconds
            })
            .send()
            .await
            .expect("Failed to show loading to user");

        Ok(())
    }

    pub async fn reply_messages(
        &self,
        messages: Vec<LineMessage>,
        reply_token: String,
    ) -> Result<reqwest::Response, &'static str> {
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.line.me/v2/bot/message/reply")
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", self.channel_access_token),
            )
            .json(&ReplyMessage {
                reply_token,
                messages,
            })
            .send()
            .await
            .expect("Failed to send chat to LINE API");

        Ok(response)
    }

    pub async fn send_messages(
        &self,
        to_user_id: String,
        messages: Vec<LineMessage>,
    ) -> Result<reqwest::Response, &'static str> {
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.line.me/v2/bot/message/push")
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", self.channel_access_token),
            )
            .json(&PushMessage {
                to: to_user_id,
                messages,
            })
            .send()
            .await
            .expect("Failed to send chat to LINE API");

        Ok(response)
    }
}
