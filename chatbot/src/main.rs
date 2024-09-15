use axum::{
    routing::{get, post},
    Router,
};
use std::env;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Welcome to UNAI API!" }))
        .route("/reply", post(reply));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn reply() -> String {
    env::var("OPENAI_API_KEY").unwrap()
}
