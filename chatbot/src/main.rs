use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    // initialize app state
    let shared_state = Arc::new(AppState::new().expect("Failed to initialize app state"));

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Welcome to UNAI API!" }))
        .route("/reply", post(reply))
        .with_state(shared_state);

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind port");
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");

    Ok(())
}

struct AppState {
    llm_client: OpenAiApi,
}

impl AppState {
    fn new() -> Result<Self, &'static str> {
        let llm_client = OpenAiApi::new().expect("Failed to initialize OpenAI API client");
        Ok(Self { llm_client })
    }
}

struct OpenAiApi {
    api_key: String,
    project_id: String,
}

impl OpenAiApi {
    fn new() -> Result<Self, &'static str> {
        let api_key =
            env::var("OPENAI_API_KEY").expect("Please set the OPENAI_API_KEY environment variable");
        let project_id = env::var("OPENAI_PROJECT_ID")
            .expect("Please set the OPENAI_PROJECT_ID environment variable");
        Ok(Self {
            api_key,
            project_id,
        })
    }
}

async fn reply(Json(payload): Json<WebhookEvent>) -> Result<String, &'static str> {
    println!("hello");
    println!("Received a request: {:?}", payload);
    Ok("hello".to_string())
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct WebhookEvent {
    destination: String,
    events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Event {
    r#type: String,
    message: Message,
    timestamp: i64,
    source: Source,
    reply_token: String,
    mode: String,
    webhook_event_id: String,
    delivery_context: DeliveryContext,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Message {
    id: String,
    r#type: String,
    text: String,
    quote_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Source {
    r#type: String,
    user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DeliveryContext {
    is_redelivery: bool,
}
