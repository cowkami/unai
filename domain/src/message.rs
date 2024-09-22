use crate::user::User;

#[derive(Debug, Clone)]
pub struct Message {
    pub from: Actor,
    pub to: Actor,
    pub text: String,
    pub reply_token: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Actor {
    User(User),
    Bot,
}
