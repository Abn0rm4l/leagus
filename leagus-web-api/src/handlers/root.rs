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
        Html(
            IndexPartialTemplate {
                title: "Dashboard",
                name: "Lionel",
                headings: vec!["Leagues", "Players", "Tables"],
            }
            .to_string(),
        )
    } else {
        Html(
            IndexFullTemplate {
                title: "Dashboard",
                name: "Lionel",
                headings: vec!["Leagues", "Players", "Tables"],
            }
            .to_string(),
        )
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexFullTemplate<'a> {
    title: &'a str,
    name: &'a str,
    headings: Vec<&'a str>,
}

#[derive(Template)]
#[template(path = "partials/index_content.html")]
struct IndexPartialTemplate<'a> {
    title: &'a str,
    name: &'a str,
    headings: Vec<&'a str>,
}
