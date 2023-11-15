use axum::{routing::get, Router};
pub fn construct_app() -> Router {
    Router::new().route("/", get(|| async { "Hello, World!" }))
}
