use domain::{Message, MessageRepo};
use reqwest::Client;

#[derive(Clone, Debug)]
pub struct MessageRepoImpl {
    http_client: Client,
}

impl MessageRepoImpl {
    pub async fn new() -> Result<Self, &'static str> {
        Ok(Self {
            http_client: Client::new(),
        })
    }
}

impl MessageRepo for MessageRepoImpl {
    async fn save(&self, messages: Vec<Message>) -> Result<(), &'static str> {
        Ok(())
    }
}
