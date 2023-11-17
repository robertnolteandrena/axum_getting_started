pub mod dto;
use std::time::Duration;

use axum::{error_handling::HandleErrorLayer, routing::get, BoxError, Router};
use hyper::StatusCode;
use tower::ServiceBuilder;

pub const TIMER_URI: &str = "/10_seconds_timer";

pub fn construct_app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            TIMER_URI,
            get(|| async {
                tokio::time::sleep(Duration::from_secs(10)).await;
                "10 seconds have passed: you may continue with other task now"
            }),
        )
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(1)),
        )
}

pub async fn handle_timeout_error(err: BoxError) -> (StatusCode, &'static str) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Handler has taken too long",
        )
    } else {
        (StatusCode::OK, "")
    }
}
