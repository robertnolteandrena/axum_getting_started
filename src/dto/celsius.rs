use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Celsius {
    pub celsius_value: f32,
}
