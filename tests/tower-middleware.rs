use std::convert::Infallible;
use std::ops::Deref;
use std::time::Duration;

use tower::{BoxError, ServiceBuilder, ServiceExt};

async fn my_service(request: usize) -> Result<i32, Infallible> {
    Ok((request + 1).try_into().unwrap())
}

#[tokio::test]
async fn test_empty_middleware() {
    let svc = tower::service_fn(my_service);
    let sb = ServiceBuilder::new().service(svc);
    let response = sb.oneshot(1).await.unwrap();
    assert_eq!(2, response)
}

#[tokio::test]
async fn test_middleware_with_one_service_and_one_layer() {
    let svc = tower::service_fn(my_service);

    // executes (2+1) * 5, because
    // layer 1, multiples with 5 -> 10
    //   service(5) -> 11
    // layer 1, does nothing on return
    let sb = ServiceBuilder::new()
        .map_request(|r: usize| r * 5)
        .service(svc);
    let response = sb.oneshot(2).await.unwrap();
    assert_eq!(11, response)
}

#[tokio::test]
async fn test_middleware_with_one_service_and_mapresponse() {
    let svc = tower::service_fn(my_service);
    // executes (2+1) * 5, because
    // layer 1, does nothing
    //   service(2) -> 3
    // layer 1, multiples with 5
    let sb = ServiceBuilder::new()
        .map_response(|r: i32| r * 5)
        .service(svc);
    let response = sb.oneshot(2).await.unwrap();
    assert_eq!(15, response)
}

async fn make_timeout_service(request: usize) -> Result<i32, Infallible> {
    tokio::time::sleep(Duration::from_secs(1)).await;
    Ok(2)
}

#[tokio::test]
async fn test_middleware_with_one_service_and_timeout() {
    let make_timeout_service = |_: usize| async move {
        tokio::time::sleep(Duration::from_secs(1)).await;
        Result::<_, BoxError>::Ok(2i32)
    };
    let svc = tower::service_fn(make_timeout_service);

    let sb = ServiceBuilder::new()
        .timeout(Duration::from_millis(50))
        .service(svc);
    let response = sb.oneshot(1).await;
    assert!(response.is_err());
    let err = response.err().unwrap();
    //assert_eq!(err.type_id(), std::any::TypeId::of::<tower::timeout::Elapsed>());
    assert!(err.is::<tower::timeout::error::Elapsed>());
}
