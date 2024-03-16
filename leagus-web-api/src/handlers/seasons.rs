use std::str::FromStr;

use askama::Template;
use axum::{extract::Path, response::Html, routing::get, Form, Json, Router};
use bson::Uuid;
use chrono::{DateTime, DurationRound, NaiveDate, NaiveDateTime, TimeDelta, Utc};
use leagus::{
    models::{League, Season},
    persistence::{mongo_store::MongoStore, WriteableStore},
};
use serde::Deserialize;
use serde_json::Value;

/// Routes available for '/seasons' path.
pub fn routes() -> Router {
    Router::new().route("/", get(list)).route(
        "/create/:league_id",
        get(get_create_season).post(post_create_season),
    )
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

pub async fn get_create_season(Path(league_id): Path<Uuid>) -> Html<String> {
    let store = MongoStore::new().await.unwrap();
    // TODO: handle when no league is found
    let league = store.get_league(&league_id).await.unwrap();

    Html(SeasonCreateModalTemplate { league }.to_string())
}

pub async fn post_create_season(
    Path(league_id): Path<Uuid>,
    Form(input): Form<CreateSeasonInput>,
) -> Html<String> {
    // TODO: Maybe store these as NaiveDate, I don't think any value is gained from having it as
    // DateTime?

    // Try parse the Date, if that doesn't work use Utc::now.
    let start = NaiveDate::parse_from_str(&input.start_date, "%Y-%m-%d");
    let start = match start {
        Ok(start) => NaiveDateTime::from(start).and_utc(),
        Err(_) => Utc::now().duration_trunc(TimeDelta::days(1)).unwrap(),
    };

    // If no end date is included we set it to +30 days from start
    // TODO: Make end date optional
    let end = NaiveDate::parse_from_str(&input.end_date, "%Y-%m-%d");
    let end = match end {
        Ok(end) => NaiveDateTime::from(end).and_utc(),
        Err(_) => (Utc::now() + TimeDelta::days(30))
            .duration_trunc(TimeDelta::days(1))
            .unwrap(),
    };

    let name = if input.season_name.is_empty() {
        // Default name will be "Month - Year".
        start.format("%B - %Y").to_string()
    } else {
        input.season_name
    };

    let mut store = MongoStore::new().await.unwrap();
    let season = Season::new(&league_id, &start, &end, &name);
    //TODO: read from input
    store.create_season(&season, input.make_active).await;

    //TODO: return something more useful
    Html(format!("{:?}", season))
}

#[derive(Template)]
#[template(path = "partials/season_create_modal.html")]
struct SeasonCreateModalTemplate {
    league: League,
}

#[derive(Deserialize)]
pub struct CreateSeasonInput {
    #[serde(default)]
    season_name: String,
    #[serde(default)]
    start_date: String,
    #[serde(default)]
    end_date: String,
    #[serde(default)]
    make_active: bool,
}
