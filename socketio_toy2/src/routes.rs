use axum::{Router, routing::get};

pub fn create_routes() -> Router<Body>{
    Router::new().route("/hello", get(|| async { "Hello, Axum!" }));
}