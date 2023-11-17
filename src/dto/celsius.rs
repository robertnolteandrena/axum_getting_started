use serde::{Deserialize, Serialize};

use super::fahrenheit::Fahrenheit;

#[derive(Debug, Deserialize, Serialize)]
pub struct Celsius {
    pub celsius_value: f32,
}
