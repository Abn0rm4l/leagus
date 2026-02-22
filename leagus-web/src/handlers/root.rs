use askama::Template;
use axum::response::Html;
use axum::{routing::get, Router};

use crate::models::league_entry::LeagueEntry;

/// Routes available for '/leagues' path.
pub fn routes() -> Router {
    // TODO: Error handling
    Router::new().route("/", get(index))
}

async fn index() -> Html<String> {
    let mut entries = vec![
        LeagueEntry {
            player: "Lionel".into(),
            score: 21,
        },
        LeagueEntry {
            player: "Robert".into(),
            score: 19,
        },
        LeagueEntry {
            player: "Caleb".into(),
            score: 35,
        },
        LeagueEntry {
            player: "Noah".into(),
            score: 35,
        },
    ];

    entries.sort_by_key(|x| -x.score);

    Html(IndexFullTemplate { entries }.to_string())
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexFullTemplate {
    entries: Vec<LeagueEntry>,
}
