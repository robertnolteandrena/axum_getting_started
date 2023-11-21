use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Extension, Router};

use crate::UserAgentValue;

pub fn get_header_routes() -> Router {
    Router::new().route("/which_user_agent", get(handler))
}

async fn handler(Extension(UserAgentValue(user_agent)): Extension<UserAgentValue>) -> Response {
    user_agent.into_response()
}
