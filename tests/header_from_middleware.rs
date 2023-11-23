use axum::http::HeaderValue;
use chrono::{DateTime, Utc};
use hands_on_lib::construct_app;
use hyper::{header::USER_AGENT, Body, Request, StatusCode};
use spectral::{assert_that, option::OptionAssertions};
use tower::ServiceExt;

#[tokio::test]
async fn header_from_middleware() {
    //arrange
    let app = construct_app();

    let request = Request::builder()
        .uri("/header/which_user_agent")
        .header(USER_AGENT, "integration test")
        .body(Body::empty())
        .unwrap();
    //
    //act
    let response = app.oneshot(request).await.unwrap();

    // assert
    let status_code = response.status();
    assert_eq!(status_code, StatusCode::OK);
    //assert that the response-timestamp header is present and parseable to a Utc DateTime
    assert_that!(response
        .headers()
        .get("response-time")
        .map(HeaderValue::to_str)
        .and_then(Result::ok)
        .map(DateTime::parse_from_rfc3339)
        .and_then(Result::ok)
        .map(Into::<DateTime<Utc>>::into))
    .is_some();
}
