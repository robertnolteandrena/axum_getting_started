use axum::http::HeaderValue;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{middleware, Extension, Router};
use hyper::header::USER_AGENT;
use hyper::{Request, StatusCode};

pub fn get_header_routes() -> Router {
    Router::new()
        .route("/which_user_agent", get(handler))
        .route_layer(middleware::from_fn(extract_user_agent))
}

async fn handler(Extension(UserAgentValue(user_agent)): Extension<UserAgentValue>) -> Response {
    user_agent.into_response()
}

#[derive(Clone)]
struct UserAgentValue(String);
async fn extract_user_agent<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let user_agent_header = req.headers().get(USER_AGENT);
    if let Some(user_agent) = user_agent_header
        .and_then(|user_agent_header: &HeaderValue| user_agent_header.to_str().ok())
        .map(String::from)
        .map(UserAgentValue)
    {
        req.extensions_mut().insert(user_agent);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
