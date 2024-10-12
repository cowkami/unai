use api_client::line::{schema::Message as LineMessage, Line, MessageClient};

#[tokio::main]
async fn main() {
    let text_message = LineMessage::text(
        "Hello, how are you?".to_string(),
        Some("528093400332239266".to_string()),
    );
    let image_message = LineMessage::image(
        "https://placehold.jp/300x200.png".to_string(),
        "https://placehold.jp/300x200.png".to_string(),
    );
    println!("Image message: {:#?}", image_message);
    let message_client = Line::new().expect("Failed to initialize Line API client");
    let response = message_client
        .send_messages(
            "U4687945e6cfdd665f019b7c0e40cf12b".to_string(),
            vec![text_message, image_message],
        )
        .await;

    println!("{:#?}", response);
}
