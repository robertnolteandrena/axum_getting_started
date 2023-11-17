use std::time::Duration;

use axum::{error_handling::HandleErrorLayer, BoxError, Router};
use hyper::StatusCode;
use tower::ServiceBuilder;

use super::MyLayer;

pub fn add_timeout_handler(router: Router, duration: Duration) -> Router {
    router.layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(handle_timeout_error))
            .timeout(duration),
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
//
