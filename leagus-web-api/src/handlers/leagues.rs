use axum::{routing::get, Json, Router};
use leagus::models::League;
use serde_json::Value;

/// Routes available for '/leagues' path.
pub fn routes() -> Router {
    Router::new()
        .route("/", get(list))
        .route("/:id", get(get_document))
}

// TODO: This will not work right now because it is not possible to use a blocking sync monogodb
// client in an asynchronous context. Exact error;
// Cannot start a runtime from within a runtime. This happens because a function (like `block_on`)
// attempted to block the current thread while the thread is being used to drive asynchronous tasks.
pub async fn list() -> Json<Value> {
    // TODO: support pagination

    // let store = MongoStore::new();
    // let leagues = store.list_leagues();
    // Json(serde_json::to_value(leagues).unwrap());

    // TODO: replace stub response with real response
    let league = League::new("stub", "stub");
    let stub_leagues = vec![league];
    Json(serde_json::to_value(stub_leagues).unwrap())
}
pub async fn get_document() {}
