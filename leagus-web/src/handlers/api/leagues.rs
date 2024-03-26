use axum::{routing::get, Json, Router};
use leagus::persistence::WriteableStore;
use leagus::{models::League, persistence::mongo_store::MongoStore};
use serde_json::Value;

/// Routes available for '/leagues' path.
pub fn routes() -> Router {
    // TODO: Error handling
    Router::new()
        .route("/", get(list))
        .route("/:id", get(get_document))
}

pub async fn list() -> Json<Value> {
    // TODO: support pagination

    // TODO: Remove the use of unwrap/expect
    // We don't want to use unwrap since that will cause the server to panic
    // which is not something we want.
    let store = MongoStore::new().await.unwrap();
    let leagues = store.list_leagues().await;
    Json(serde_json::to_value(leagues).unwrap())
}

/// Get league by id
pub async fn get_document() -> Json<Value> {
    let league = League::new("Stub League", "For all the stubs");
    Json(serde_json::to_value(league).unwrap())
}
