pub mod controllers;
pub mod dto;
pub mod layers;
use std::time::Duration;

use axum::Router;
use controllers::{
    header_controller::get_header_routes, temperature_controller::get_temperature_routes,
};
use layers::timeout_layer::add_timeout_handler;

pub fn construct_app() -> Router {
    let mut app = Router::new()
        .nest("/temperature", get_temperature_routes())
        .nest("/header", get_header_routes());
    app = add_timeout_handler(app, Duration::from_secs(1));
    app
}
