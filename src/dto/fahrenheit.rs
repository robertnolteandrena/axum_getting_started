use serde::{Deserialize, Serialize};

use super::celsius::Celsius;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Fahrenheit {
    pub fahrenheit_value: f32,
}
impl From<Fahrenheit> for Celsius {
    fn from(val: Fahrenheit) -> Self {
        let celsius_value = (val.fahrenheit_value - 32f32) * 5f32 / 9f32;
        Celsius { celsius_value }
    }
}
