use std::time::Duration;

use axum::{error_handling::HandleErrorLayer, routing::get, BoxError, Router};
use hyper::StatusCode;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    let app = construct_app();
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn construct_app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/10_seconds_timer",
            get(|| async {
                tokio::time::sleep(Duration::from_secs(10)).await;
                "10 seconds have passed: you may continue with other task now"
            }),
        )
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(1)),
        )
}

async fn handle_timeout_error(err: BoxError) -> (StatusCode, &'static str) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Handler has taken too long",
        )
    } else {
        (StatusCode::OK, "")
    }
}
#[cfg(test)]
mod tests {
    use crate::construct_app;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
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

    #[tokio::test]
    async fn timeout() {
        let app = construct_app();

        let response = app
            .oneshot(Request::builder().uri("/10_seconds_timer").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

}
