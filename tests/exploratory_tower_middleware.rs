use std::convert::Infallible;

use spectral::assert_that;
use tower::{ServiceBuilder, ServiceExt};

async fn my_service(request: String) -> Result<String, Infallible> {
    let response = format!("my_service({})", request.to_owned());
    Ok(response)
}

#[tokio::test]
async fn middleware_with_one_service() {
    //a service consumes a request and returns a response
    let svc = tower::service_fn(my_service);
    let sb = ServiceBuilder::new().service(svc);
    let response = sb.oneshot("Vanilla request".to_owned()).await.unwrap();
    assert_that!(response).is_equal_to("my_service(Vanilla request)".to_owned());
}

#[tokio::test]
async fn middleware_with_one_service_and_one_layer() {
    let svc = tower::service_fn(my_service);
    let sb = ServiceBuilder::new()
        .map_request(|request: String| format!("map_request({})", request))
        .service(svc);
    let response = sb.oneshot("Vanilla request".to_owned()).await.unwrap();
    assert_that!(response).is_equal_to("my_service(map_request(Vanilla request))".to_owned());
}

#[tokio::test]
async fn middleware_with_one_service_and_mapresponse() {
    let svc = tower::service_fn(my_service);
    let sb = ServiceBuilder::new()
        .map_response(|response: String| format!("map_response({})", response))
        .service(svc);
    let response = sb.oneshot("Vanilla request".to_owned()).await.unwrap();
    assert_that!(response).is_equal_to("map_response(my_service(Vanilla request))".to_owned());
}
