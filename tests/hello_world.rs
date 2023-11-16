//Integration test
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use hands_on_lib::construct_app;
use tower::ServiceExt;

#[tokio::test]
async fn hello_world() {
    // arrange
    let app = construct_app();
    let request = Request::builder().uri("/").body(Body::empty()).unwrap();

    // act
    let response = app.oneshot(request).await.unwrap();

    // assert
    let status_code = response.status();
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();

    assert_eq!(status_code, StatusCode::OK);
    assert_eq!(&body[..], b"Hello, World!");
}
