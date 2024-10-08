use api_client::gpt::Gpt;

#[tokio::main]
async fn main() {
    let llm_client = Gpt::new().expect("Failed to initialize OpenAI API client");
    let response = llm_client
        .chat("レシピのアイデアを10個考えてちょ".to_string())
        .await;
    println!("{:#?}", response);
}
