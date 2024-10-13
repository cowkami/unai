use crate::context::Context;
use crate::user::User;

#[derive(Debug, Clone)]
pub struct Message {
    pub user: User,
    pub from: Actor,
    pub text: String,
    pub context: Option<Context>,
    pub reply_token: Option<String>,
    pub image: Option<ImageMessage>,
}

#[derive(Debug, Clone)]
pub enum Actor {
    User,
    Bot,
}

#[derive(Debug, Clone)]
pub struct ImageMessage {
    pub url: String,
    pub preview_url: String,
}
