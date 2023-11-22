pub mod controllers;
pub mod dto;
use std::time::Duration;

use axum::{
    error_handling::HandleErrorLayer,
    http::HeaderValue,
    middleware::{self},
    BoxError, Router,
};
use controllers::{
    header_controller::get_header_routes, temperature_controller::get_temperature_routes,
};
use hyper::{header::USER_AGENT, StatusCode};
use tower::ServiceBuilder;

pub fn construct_app() -> Router {
    Router::new()
        .nest("/temperature", get_temperature_routes())
        .nest("/header", get_header_routes())
        .route_layer(
            ServiceBuilder::new()
                .layer(middleware::from_fn(extract_user_agent))
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

#[derive(Clone)]
pub struct UserAgentValue(pub String);
async fn extract_user_agent<B>(
    mut req: hyper::Request<B>,
    next: axum::middleware::Next<B>,
) -> Result<axum::response::Response, hyper::StatusCode> {
    let user_agent_header = req.headers().get(USER_AGENT);
    if let Some(user_agent) = user_agent_header
        .and_then(|user_agent_header: &HeaderValue| user_agent_header.to_str().ok())
        .map(String::from)
        .map(UserAgentValue)
    {
        req.extensions_mut().insert(user_agent);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
