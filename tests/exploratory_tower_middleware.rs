use std::{convert::Infallible, fmt::Display};

use spectral::assert_that;
use tower::{ServiceBuilder, ServiceExt};

async fn my_service<T: Display>(request: T) -> Result<String, Infallible> {
    let response = format!("my_service({})", request);
    Ok(response)
}

#[tokio::test]
async fn middleware_with_one_service() {
    let sb = ServiceBuilder::new().service_fn(my_service);
    let response = sb.oneshot("Vanilla request").await.unwrap();
    assert_that!(response.as_ref()).is_equal_to("my_service(Vanilla request)");
}

#[tokio::test]
async fn middleware_with_one_service_and_one_layer() {
    let sb = ServiceBuilder::new()
        .map_request(|request: &str| format!("map_request({})", request))
        .service_fn(my_service);
    let response = sb.oneshot("Vanilla request").await.unwrap();
    assert_that!(response.as_ref()).is_equal_to("my_service(map_request(Vanilla request))");
}

#[tokio::test]
async fn middleware_with_different_return_type() {
    //a service consumes a request and returns a response
    let sb = ServiceBuilder::new().service_fn(my_service);
    let response = sb.oneshot(42).await.unwrap();
    assert_that!(response.as_ref()).is_equal_to("my_service(42)");
}

#[tokio::test]
async fn middleware_with_one_service_and_mapresponse() {
    let sb = ServiceBuilder::new()
        .map_response(|response: String| format!("map_response({})", response))
        .service_fn(my_service);
    let response = sb.oneshot("Vanilla request").await.unwrap();
    assert_that!(response.as_ref()).is_equal_to("map_response(my_service(Vanilla request))");
}
