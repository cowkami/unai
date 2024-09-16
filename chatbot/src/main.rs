mod line;
mod openai;

use axum::{
    routing::{get, post},
    Json, Router,
};
use line::EventType;
use openai::OpenAiApi;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    // initialize logger
    env_logger::init();

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

async fn reply(Json(payload): Json<line::WebhookEvent>) -> Result<String, &'static str> {
    log::trace!("Received payload: {:#?}", payload);
    payload.events.iter().for_each(|event| {
        match event.r#type {
            EventType::Message => {
                log::info!("Received message: {}", event.message.text);
            }
        };
    });
    Ok("hello".to_string())
}
