use api_client::gpt::Gpt;
use domain::UserDemand;

#[tokio::main]
async fn main() {
    let llm_client = Gpt::new().expect("Failed to initialize OpenAI API client");

    let text = "レシピを10個考えて";
    let user_demand = llm_client.detect_demand(text.to_string()).await;
    assert!(
        matches!(user_demand, Ok(UserDemand::Chat)),
        "Expected Chat, got {:?}",
        user_demand
    );

    let text = "ミーアキャットが逆立ちしている画像をつくって";
    let user_demand = llm_client.detect_demand(text.to_string()).await;
    assert!(
        matches!(user_demand, Ok(UserDemand::CreateImage)),
        "Expected CreateImage, got {:?}",
        user_demand
    );

    println!("All tests passed!");
}
