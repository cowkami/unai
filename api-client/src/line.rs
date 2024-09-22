pub mod schema;

use schema::{LoadingStart, Message, ReplyMessage};

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

    pub async fn show_loading(&self) -> Result<(), reqwest::Error> {
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
            .await?;

        Ok(())
    }

    pub async fn reply(&self, chat: &str, reply_token: String) -> Result<(), reqwest::Error> {
        let client = reqwest::Client::new();
        client
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

        Ok(())
    }
}
