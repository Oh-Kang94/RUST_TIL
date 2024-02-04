use axum::{Server, routing::{ get, post }, http::StatusCode, Json, Router };
use serde::Serialize;
use serde_json::Value;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/chat", post(send_data));

    let addr = "0.0.0.0:8080".parse().unwrap();
    Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
