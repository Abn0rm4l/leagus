use axum::{routing::get, Json, Router};
use leagus::persistence::{mongo_store::MongoStore, WriteableStore};
use serde_json::Value;

/// Routes available for '/seasons' path.
pub fn routes() -> Router {
    Router::new().route("/", get(list))
}

/// List all seasons
pub async fn list() -> Json<Value> {
    // TODO: support pagination
    // TODO: Remove the use of unwrap/expect
    // We don't want to use unwrap since that will cause the server to panic
    // which is not something we want.
    let store = MongoStore::new("mongodb://root:example@127.0.0.1:27017")
        .await
        .unwrap();
    let seasons = store.list_seasons().await;
    Json(serde_json::to_value(seasons).unwrap())
}
