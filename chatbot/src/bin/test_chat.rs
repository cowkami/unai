use chatbot::gpt::Gpt;

#[tokio::main]
async fn main() {
    let llm_client = Gpt::new().expect("Failed to initialize OpenAI API client");
    let response = llm_client.send_chat("Hello, how are you?").await;
    println!("{:#?}", response);
}
