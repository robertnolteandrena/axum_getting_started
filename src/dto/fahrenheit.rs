use serde::{Deserialize, Serialize};

use super::celsius::Celsius;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Fahrenheit {
    pub fahrenheit_value: f32,
}
impl From<Celsius> for Fahrenheit {
    fn from(val: Celsius) -> Self {
        let fahrenheit_value = val.celsius_value * 9f32 / 5f32 + 32f32;
        Fahrenheit { fahrenheit_value }
    }
}
