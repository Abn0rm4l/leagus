use askama::Template;
use axum::{response::Html, routing::get, Json, Router};
use leagus::persistence::{mongo_store::MongoStore, WriteableStore};
use serde_json::Value;

/// Routes available for '/seasons' path.
pub fn routes() -> Router {
    Router::new()
        .route("/", get(list))
        .route("/create", get(create_season_modal))
}

/// List all seasons
pub async fn list() -> Json<Value> {
    // TODO: support pagination
    // TODO: Remove the use of unwrap/expect
    // We don't want to use unwrap since that will cause the server to panic
    // which is not something we want.
    let store = MongoStore::new().await.unwrap();
    let seasons = store.list_seasons().await;
    Json(serde_json::to_value(seasons).unwrap())
}

pub async fn create_season_modal() -> Html<String> {
    Html(SeasonCreateModalTemplate { title: "None" }.to_string())
}

#[derive(Template)]
#[template(path = "partials/season_create_modal.html")]
struct SeasonCreateModalTemplate<'a> {
    title: &'a str,
}
