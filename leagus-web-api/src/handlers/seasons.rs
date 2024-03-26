use askama::Template;
use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Form, Json, Router,
};
use bson::Uuid;
use chrono::{DurationRound, NaiveDate, NaiveDateTime, TimeDelta, Utc};
use leagus::{
    models::{League, LeagueId, Season},
    persistence::WriteableStore,
};
use serde::Deserialize;
use serde_json::Value;

use crate::{errors::LeagusError, state::AppState};

/// Routes available for '/seasons' path.
pub fn routes<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/", get(list))
        .route(
            "/create/:league_id",
            get(get_create_season).post(post_create_season),
        )
        .with_state(state)
}

/// List all seasons
pub async fn list(State(state): State<AppState>) -> Json<Value> {
    // TODO: support pagination
    // TODO: Remove the use of unwrap/expect
    // We don't want to use unwrap since that will cause the server to panic
    // which is not something we want.
    let store = &state.store;
    let seasons = store.list_seasons().await;
    Json(serde_json::to_value(seasons).unwrap())
}

/// Get the create new season form
pub async fn get_create_season(
    State(state): State<AppState>,
    Path(league_id): Path<Uuid>,
) -> Result<Html<String>, LeagusError> {
    let store = &state.store;
    let league_id = LeagueId::from(league_id);
    // TODO: handle when no league is found
    let league = store.get_league(&league_id).await.unwrap();

    Ok(Html(SeasonCreateModalTemplate { league }.to_string()))
}

/// Create a new [`Season`]
pub async fn post_create_season(
    State(state): State<AppState>,
    Path(league_id): Path<Uuid>,
    Form(input): Form<CreateSeasonInput>,
) -> Result<Html<String>, LeagusError> {
    println!("Create Season Input: {:?}", input);

    // TODO: Maybe store these as NaiveDate, I don't think any value is gained from having it as
    // DateTime?
    //
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

    let make_active = input.make_active.eq_ignore_ascii_case("on");

    let store = &state.store;
    let league_id = LeagueId::from(league_id);
    let season = Season::new(&league_id, &start, &end, &name);
    store.create_season(&season, make_active).await;

    //TODO: return something more useful, maybe a sucess dialog?
    Ok(Html(format!("{:?}", season)))
}

#[derive(Template)]
#[template(path = "partials/season_create_modal.html")]
struct SeasonCreateModalTemplate {
    league: League,
}

#[derive(Deserialize, Debug)]
pub struct CreateSeasonInput {
    #[serde(default)]
    season_name: String,
    #[serde(default)]
    start_date: String,
    #[serde(default)]
    end_date: String,
    #[serde(default)]
    make_active: String,
}
