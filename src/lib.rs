pub mod controllers;
pub mod dto;
use std::time::Duration;

use axum::{error_handling::HandleErrorLayer, BoxError, Router};
use controllers::temperature_controller::get_temperature_routes;
use hyper::StatusCode;
use tower::ServiceBuilder;

pub fn construct_app() -> Router {
    Router::new()
        .nest("/temperature", get_temperature_routes())
        .route_layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(10)),
        )
}
async fn handle_timeout_error(err: BoxError) -> (StatusCode, &'static str) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Handler has taken too long",
        )
    } else {
        (StatusCode::OK, "")
    }
}
