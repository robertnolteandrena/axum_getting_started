use axum::{routing::get, Router};
pub fn construct_app() -> Router {
    Router::new().route("/", get(|| async { "Hello, World!" }))
}

//Unit test
#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use crate::construct_app;

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
}
