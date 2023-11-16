use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use hands_on_lib::construct_app;
use tower::util::ServiceExt;

#[tokio::test]
async fn timeout() {
    //arrange
    let app = construct_app();

    let request = Request::builder()
        .uri("/10_seconds_timer")
        .body(Body::empty())
        .unwrap();
    //act
    let response = app.oneshot(request).await.unwrap();

    //assert
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
