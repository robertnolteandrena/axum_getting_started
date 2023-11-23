use serde::{Deserialize, Serialize};

use super::celsius::Celsius;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Fahrenheit {
    pub fahrenheit_value: f32,
}
impl From<Celsius> for Fahrenheit {
    ///Converts and consumes a Celsius instance to a Fahrenheit instance.
    ///
    ///# Example
    ///```
    ///use hands_on_lib::dto::celsius::Celsius;
    ///use hands_on_lib::dto::fahrenheit::Fahrenheit;
    ///let celsius=Celsius {celsius_value : 0f32};
    ///let fahrenheit= Fahrenheit::from(celsius);
    ///assert_eq!(fahrenheit.fahrenheit_value, 32f32);
    ///```
    ///
    ///
    ///
    fn from(val: Celsius) -> Self {
        let fahrenheit_value = val.celsius_value * 9f32 / 5f32 + 32f32;
        Fahrenheit { fahrenheit_value }
    }
}
