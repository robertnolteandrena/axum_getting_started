//Integration test
use axum::http::{Request, StatusCode};
use hands_on_lib::{
    construct_app,
    dto::{celsius::Celsius, fahrenheit::Fahrenheit},
};
use hyper::{header::CONTENT_TYPE, Body};
use mime::APPLICATION_JSON;
use tower::ServiceExt;

#[tokio::test]
async fn from_celsius_to_fahrenheit() {
    // arrange
    let celsius_temperature = Celsius {
        celsius_value: 37.7778f32,
    };
    let app = construct_app();
    let json_value = serde_json::to_value(celsius_temperature).unwrap();
    let request_body = Body::from(json_value.to_string());
    let request = Request::builder()
        .uri("/temperature/fahrenheit")
        .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
        .body(request_body)
        .unwrap();

    // act
    let response = app.oneshot(request).await.unwrap();

    // assert
    let status_code = response.status();
    let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let fahrenheit_temperature: Fahrenheit = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(status_code, StatusCode::OK);
    assert_eq!(
        fahrenheit_temperature,
        Fahrenheit {
            fahrenheit_value: 100f32
        }
    );
}
