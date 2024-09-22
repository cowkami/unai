mod app;

use api_client::line;
use app::App;
use axum::{
    extract::Extension,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    // initialize logger
    env_logger::init();

    // initialize app
    let app = App::new().expect("Failed to initialize app");

    // build our application with a single route
    let router = Router::new()
        .route("/", get(|| async { "Welcome to UNAI API!" }))
        .route("/conversation", post(conversation))
        .layer(Extension(app));

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind port");

    axum::serve(listener, router)
        .await
        .expect("Server failed to start");

    Ok(())
}

async fn conversation(
    Extension(app): Extension<App>,
    Json(payload): Json<line::schema::WebhookEvent>,
) -> Result<StatusCode, &'static str> {
    log::trace!("Received payload: {:#?}", payload);

    tokio::spawn(async move {
        // filter out message events
        let message_event = payload
            .events
            .iter()
            .filter(|event| matches!(event.r#type, line::schema::EventType::Message))
            .next() // get the first message event
            .expect("No message event found");

        log::info!(
            "User message:\n\
        user_id: {}\n\
        text: {}",
            message_event.source.user_id,
            message_event.message.text,
        );

        // show loading to LINE
        app.message_client
            .show_loading()
            .await
            .expect("Failed to show loading");

        // send chat
        let bot_response = app
            .llm_client
            .send_chat(&message_event.message.text)
            .await
            .expect("Failed to send chat to OpenAI API");

        log::info!(
            "Bot message: \n\
        text: {}",
            bot_response,
        );

        // reply chat to LINE
        app.message_client
            .reply(bot_response.as_str(), message_event.reply_token.clone())
            .await
            .expect("Failed to send chat to LINE API");
    });

    Ok(StatusCode::OK)
}
