use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Extension, Router};

use crate::layers::user_agent_extractor::{add_user_agent_extractor_layer, UserAgentValue};

pub fn get_header_routes() -> Router {
    let mut router = Router::new().route("/which_user_agent", get(handler));
    router = add_user_agent_extractor_layer(router);
    router
}

async fn handler(Extension(UserAgentValue(user_agent)): Extension<UserAgentValue>) -> Response {
    user_agent.into_response()
}
