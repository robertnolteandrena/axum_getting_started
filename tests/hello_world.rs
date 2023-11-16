//Integration test
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use hands_on_lib::construct_app;
use tower::ServiceExt;

#[tokio::test]
async fn hello_world() {
    let app = construct_app();

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"Hello, World!");
}
