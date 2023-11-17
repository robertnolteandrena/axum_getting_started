use axum::http::HeaderValue;
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::Router;
use hyper::header::USER_AGENT;
use hyper::{Request, StatusCode};

pub fn add_user_agent_extractor_layer(router: Router) -> Router {
    router.route_layer(middleware::from_fn(extract_user_agent))
}
#[derive(Clone)]
pub struct UserAgentValue(pub String);
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
