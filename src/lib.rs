pub mod controllers;
pub mod dto;
pub mod layers;
use std::time::Duration;

use axum::Router;
use controllers::temperature_controller::get_temperature_routes;
use layers::timeout_layer::add_timeout_handler;

pub fn construct_app() -> Router {
    let mut app = Router::new().nest("/temperature", get_temperature_routes());
    app = add_timeout_handler(app, Duration::from_secs(1));
    app
}
