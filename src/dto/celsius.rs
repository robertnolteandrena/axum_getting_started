use serde::{Deserialize, Serialize};

use super::fahrenheit::Fahrenheit;

#[derive(Debug, Deserialize, Serialize)]
pub struct Celsius {
    pub celsius_value: f32,
}
impl From<Celsius> for Fahrenheit {
    fn from(val: Celsius) -> Self {
        let fahrenheit_value = val.celsius_value * 9f32 / 5f32 + 32f32;
        Fahrenheit { fahrenheit_value }
    }
}
