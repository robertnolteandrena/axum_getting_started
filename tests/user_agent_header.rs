//Integration test
use axum::http::{Request, StatusCode};
use hands_on_lib::construct_app;
use hyper::{header::USER_AGENT, Body};
use tower::ServiceExt;

const USER_AGENT_VALUE: &str = "Also look into axum::extract::TypedHeader";

#[tokio::test]
async fn respond_with_header() {
    // arrange
    let app = construct_app();
    let request = Request::builder()
        .uri("/header/which_user_agent")
        .header(USER_AGENT, USER_AGENT_VALUE)
        .body(Body::empty())
        .unwrap();

    // act
    let response = app.oneshot(request).await.unwrap();

    // assert
    let status_code = response.status();
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();

    assert_eq!(status_code, StatusCode::OK);
    assert_eq!(&body[..], USER_AGENT_VALUE.as_bytes());
}
