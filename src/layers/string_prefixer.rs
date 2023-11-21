use std::{convert::Infallible, fmt::Display};

use axum::{
    middleware::{self, Next},
    response::{IntoResponse, Response},
    Router,
};
use hyper::body::HttpBody;
use tower::ServiceBuilder;

pub fn add_string_prefixer(router: Router) -> Router {
    let sb = ServiceBuilder::new().map_request(|request: hyper::Request<hyper::Body>| {
        let prefixed = format!("prefix {:?}", request.into_body());
        hyper::Request::new(prefixed.into())
    });

    router.layer(sb)
}
