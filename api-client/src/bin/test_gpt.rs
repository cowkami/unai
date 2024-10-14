use api_client::gpt::Gpt;
use domain::{Actor, Message, User};

#[tokio::main]
async fn main() {
    let llm_client = Gpt::new().expect("Failed to initialize OpenAI API client");
    let response = llm_client
        .chat(vec![Message {
            user: User {
                id: "1234567890".to_string(),
            },
            from: Actor::User,
            text: "レシピのアイデアを10個考えてちょ".to_string(),
            context: None,
            reply_token: None,
            image: None,
        }])
        .await;
    println!("{:#?}", response);
}
