pub mod schema;

use schema::{Message, ReplyMessage};

#[derive(Clone)]
pub struct Line {
    channel_access_token: String,
}

impl Line {
    pub fn new() -> Result<Self, &'static str> {
        let channel_access_token = std::env::var("LINE_CHANNEL_ACCESS_TOKEN")
            .expect("Please set the LINE_CHANNEL_ACCESS_TOKEN environment variable");

        Ok(Self {
            channel_access_token,
        })
    }

    pub async fn reply(&self, chat: &str, reply_token: String) -> Result<(), reqwest::Error> {
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
                messages: vec![Message {
                    r#type: "text".to_string(),
                    text: chat.to_string(),
                    id: None,
                    quote_token: None,
                }],
            })
            .send()
            .await?;

        log::trace!("Received reply API response: {:#?}", response);

        Ok(())
    }
}
