mod line;
mod openai;

use axum::{
    extract::Extension,
    routing::{get, post},
    Json, Router,
};
use line::EventType;
use openai::OpenAiApi;

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    // initialize logger
    env_logger::init();

    // initialize app context
    let app_context = AppContext::new().expect("Failed to initialize app context");

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Welcome to UNAI API!" }))
        .route("/reply", post(reply))
        .layer(Extension(app_context));

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind port");
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");

    Ok(())
}

#[derive(Clone)]
struct AppContext {
    llm_client: OpenAiApi,
}

impl AppContext {
    fn new() -> Result<Self, &'static str> {
        let llm_client = OpenAiApi::new().expect("Failed to initialize OpenAI API client");
        Ok(Self { llm_client })
    }
}

async fn reply(
    Extension(app_context): Extension<AppContext>,
    Json(payload): Json<line::WebhookEvent>,
) -> Result<String, &'static str> {
    log::trace!("Received payload: {:#?}", payload);

    // filter out message events
    let message_event = payload
        .events
        .iter()
        .filter(|event| matches!(event.r#type, EventType::Message))
        .next()
        .expect("No message event found");

    log::info!(
        "Received message: \n\
        user_id: {}\n\
        text: {}",
        message_event.source.user_id,
        message_event.message.text,
    );

    Ok("hello".to_string())
}
