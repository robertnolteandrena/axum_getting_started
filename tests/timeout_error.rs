use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use hands_on_lib::construct_app;
use tower::util::ServiceExt;
#[tokio::test]
async fn timeout() {
    let app = construct_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/10_seconds_timer")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
