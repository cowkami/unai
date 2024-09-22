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
        app.conversation(payload)
            .await
            .expect("Failed to process conversation");
    });

    Ok(StatusCode::OK)
}
