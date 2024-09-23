use api_client::gpt::Gpt;

#[tokio::main]
async fn main() {
    let llm_client = Gpt::new().expect("Failed to initialize OpenAI API client");

    let text = "ミーアキャットが逆立ちしている画像をつくって";
    let image = llm_client.generate_image(text.to_string()).await;

    println!("{:#?}", image);
}
