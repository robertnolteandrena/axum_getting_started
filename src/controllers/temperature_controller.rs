use axum::extract::Json;
use axum::routing::get;
use axum::Router;

use crate::dto::celsius::Celsius;
use crate::dto::fahrenheit::Fahrenheit;

pub fn get_temperature_routes() -> Router {
    Router::new().route(
        "/fahrenheit",
        get(|Json(celsius): Json<Celsius>| async {
            //make use of the From trait implementation
            let fahrenheit_temperature: Fahrenheit = celsius.into();
            Json(fahrenheit_temperature)
        }),
    )
}
