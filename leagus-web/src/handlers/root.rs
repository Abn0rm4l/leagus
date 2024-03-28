use askama::Template;
use axum::response::Html;
use axum::{routing::get, Router};
use axum_htmx::HxBoosted;

/// Routes available for '/leagues' path.
pub fn routes() -> Router {
    // TODO: Error handling
    Router::new().route("/", get(index))
}

async fn index(HxBoosted(boosted): HxBoosted) -> Html<String> {
    if boosted {
        Html(IndexPartialTemplate { name: "Lionel" }.to_string())
    } else {
        Html(IndexFullTemplate { name: "Lionel" }.to_string())
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexFullTemplate<'a> {
    name: &'a str,
}

#[derive(Template)]
#[template(path = "partials/index_content.html")]
struct IndexPartialTemplate<'a> {
    name: &'a str,
}
