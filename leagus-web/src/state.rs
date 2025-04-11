use leagus::persistence::mongo_store::MongoStore;

/// Contains the global state of the app to be shared across requests
#[derive(Clone)]
pub struct AppState {
    pub store: MongoStore,
}

impl AppState {
    pub async fn new() -> AppState {
        AppState {
            store: MongoStore::new("mongodb://root:example@127.0.0.1:27017")
                .await
                .unwrap(),
        }
    }
}
