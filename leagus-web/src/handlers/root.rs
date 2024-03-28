use askama::Template;
use axum::response::Html;
use axum::{routing::get, Router};

/// Routes available for '/leagues' path.
pub fn routes() -> Router {
    // TODO: Error handling
    Router::new().route("/", get(index))
}

async fn index() -> Html<String> {
    Html(IndexFullTemplate { name: "Lionel" }.to_string())
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexFullTemplate<'a> {
    name: &'a str,
}
