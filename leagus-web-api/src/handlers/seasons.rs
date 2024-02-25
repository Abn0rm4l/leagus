use axum::{routing::get, Json, Router};
use chrono::{TimeDelta, Utc};
use leagus::models::{LeagueId, Season};
use serde_json::Value;

/// Routes available for '/seasons' path.
pub fn routes() -> Router {
    Router::new().route("/", get(list))
}

/// List all seasons
pub async fn list() -> Json<Value> {
    // TODO: support pagination
    // TODO: replace stub response with real response
    let season = Season::new(
        &LeagueId::new(),
        &Utc::now(),
        &(Utc::now() + TimeDelta::days(30)),
        Some("Stub season".to_string()),
    );

    let stub_seasons = vec![season];
    Json(serde_json::to_value(stub_seasons).unwrap())
}
