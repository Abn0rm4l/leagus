use axum::{routing::get, Router};

/// Routes available for '/sessions' path.
pub fn routes() -> Router {
    Router::new().route("/", get(list))
}

pub async fn list() {}
