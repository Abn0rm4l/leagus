use askama::Template;
use axum::extract::{Path, State};
use axum::response::Html;
use axum::{routing::get, Router};
use axum_htmx::{HxBoosted, HxRequest};
use bson::Uuid;
use leagus::models::{
    League, LeagueId, ParticipantId, PointsTable, PointsTableEntry, Season, Session,
};
use leagus::persistence::WriteableStore;

use crate::errors::LeagusError;
use crate::state::AppState;

/// Routes available for '/leagues' path.
pub fn routes<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/", get(list))
        .route("/:league_id", get(get_by_id))
        .with_state(state)
}

async fn list(State(state): State<AppState>) -> Result<Html<String>, LeagusError> {
    let store = &state.store;
    let leagues = store.list_leagues().await;

    Ok(Html(LeaguesFullTemplate { leagues }.to_string()))
}

async fn get_by_id(
    State(state): State<AppState>,
    HxRequest(hxrequest): HxRequest,
    HxBoosted(boosted): HxBoosted,
    Path(league_id): Path<Uuid>,
) -> Result<Html<String>, LeagusError> {
    println!("Getting page for league: {league_id}");

    let store = &state.store;
    let league_id = LeagueId::from(league_id);
    println!("Converted league_id to {league_id}");
    let league = store.get_league(&league_id).await;
    println!("Found league: {:?}", league);
    let seasons = store.list_seasons_for_league(&league_id).await;

    // TODO: Provide real data
    let mut entries = vec![
        PointsTableEntry {
            participant_name: "Roger".to_string(),
            participant_id: ParticipantId::new(),
            points: 189,
            wins: 31,
            losses: 15,
        },
        PointsTableEntry {
            participant_name: "Rafael".to_string(),
            participant_id: ParticipantId::new(),
            points: 193,
            wins: 32,
            losses: 18,
        },
        PointsTableEntry {
            participant_name: "Andy".to_string(),
            participant_id: ParticipantId::new(),
            points: 163,
            wins: 28,
            losses: 19,
        },
    ];

    entries.sort_by(|a, b| a.points.cmp(&b.points).reverse());
    let points_table = PointsTable { entries };

    match league {
        None => Err(LeagusError::Internal), // TODO: Return better error
        Some(league) => {
            let active_season = seasons
                .iter()
                .find(|x| Some(x.id) == league.active_season)
                .cloned();

            let active_session_id = active_season.clone().and_then(|x| x.active_session);

            let active_session = match active_session_id {
                Some(id) => store.get_session(&id).await,
                _ => None,
            };

            if boosted || hxrequest {
                Ok(Html(
                    LeagueContentTemplate {
                        league,
                        seasons,
                        points_table,
                        active_season,
                        active_session,
                    }
                    .to_string(),
                ))
            } else {
                Ok(Html(
                    LeagueTemplate {
                        league,
                        seasons,
                        points_table,
                        active_season,
                        active_session,
                    }
                    .to_string(),
                ))
            }
        }
    }
}

// -- Templates

#[derive(Template)]
#[template(path = "leagues.html")]
struct LeaguesFullTemplate {
    leagues: Vec<League>,
}

#[derive(Template)]
#[template(path = "partials/leagues/single_content.html")]
struct LeagueContentTemplate {
    league: League,
    seasons: Vec<Season>,
    active_season: Option<Season>,
    active_session: Option<Session>,
    points_table: PointsTable,
}

#[derive(Template)]
#[template(path = "league.html")]
struct LeagueTemplate {
    league: League,
    seasons: Vec<Season>,
    active_season: Option<Season>,
    active_session: Option<Session>,
    points_table: PointsTable,
}
