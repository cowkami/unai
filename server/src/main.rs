mod app;

use api_client::line;
use api_client::{gcs::Gcs, gpt::Gpt, line::Line};
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
    let llm_client = Gpt::new().expect("Failed to initialize OpenAI API client");
    let message_client = Line::new().expect("Failed to initialize LINE client");
    let storage_client = Gcs::new()
        .await
        .expect("Failed to initialize Cloud Storage client");
    let app = App {
        llm_client,
        message_client,
        storage_client,
    };

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
    Extension(app): Extension<App<Gpt, Line, Gcs>>,
    Json(payload): Json<line::schema::WebhookEvent>,
) -> Result<StatusCode, &'static str> {
    log::trace!("Received payload: {:#?}", payload);

    tokio::spawn(async move {
        app.conversation(payload)
            .await
            .expect("Failed to process conversation");
    });

    Ok(StatusCode::OK)
}
