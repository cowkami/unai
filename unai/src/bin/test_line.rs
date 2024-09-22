use chatbot::line::Line;

#[tokio::main]
async fn main() {
    let message_client = Line::new().expect("Failed to initialize Line API client");
    let response = message_client
        .reply(
            "Hello, how are you?",
            "13912a8f297d4fa5bea46f5bcd03727a".to_string(),
        )
        .await;

    println!("{:#?}", response);
}
