use axum::extract::{Path, Query, State};
use axum::response::Html;
use axum::routing::post;
use axum::Form;
use axum::{routing::get, Router};
use axum_htmx::{HxBoosted, HxRequest};
use bson::Uuid;
use leagus::models::{
    League, LeagueId, ParticipantId, PointsTable, PointsTableEntry, Season, SeasonId,
};
use leagus::persistence::WriteableStore;
use serde::Deserialize;
use tracing::{debug, info};

use crate::errors::LeagusError;
use crate::state::AppState;
use crate::templates::{
    LeagueContentTemplate, LeagueTemplate, LeaguesFullTemplate, LeaguesListTemplate,
    PointsTableTemplate, SeasonsForLeagueTemplate, SessionTemplate,
};

/// Routes available for '/leagues' path.
pub fn routes<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/", get(list))
        .route("/:league_id", get(get_league))
        .route("/:league_id/seasons", get(get_seasons_for_league))
        .route("/create", post(post_create_league))
        .with_state(state)
}

async fn list(State(state): State<AppState>) -> Result<Html<String>, LeagusError> {
    let store = &state.store;
    let leagues = store.list_leagues().await;

    Ok(Html(LeaguesFullTemplate { leagues }.to_string()))
}

async fn get_league(
    State(state): State<AppState>,
    HxRequest(hxrequest): HxRequest,
    HxBoosted(boosted): HxBoosted,
    params: Option<Query<GetQueryParams>>,
    Path(league_id): Path<Uuid>,
) -> Result<Html<String>, LeagusError> {
    //TODO: Add proper logging
    tracing::debug!("Getting page for league: {league_id}");
    tracing::debug!("with query params {:?}", params);

    let store = &state.store;
    let league_id = LeagueId::from(league_id);
    tracing::debug!("Converted league_id to {league_id}");
    let league = store.get_league(&league_id).await;
    tracing::debug!("Found league: {:?}", league);
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
    let points_table_template = PointsTableTemplate { points_table };

    match league {
        None => Err(LeagusError::Internal), // TODO: Return better error
        Some(league) => {
            let active_season = get_target_season(&seasons, &league, params.as_ref());
            let active_session_id = active_season.clone().and_then(|x| x.active_session);

            let active_session = match active_session_id {
                Some(id) => store.get_session(&id).await,
                _ => None,
            };

            let session_template = SessionTemplate {
                active_session,
                active_season: active_season.clone(),
            };

            let league_content_template = LeagueContentTemplate {
                league,
                seasons,
                active_season,
                points_table_template,
                session_template,
            };

            if boosted || hxrequest {
                Ok(Html(league_content_template.to_string()))
            } else {
                Ok(Html(
                    LeagueTemplate {
                        league_content_template,
                    }
                    .to_string(),
                ))
            }
        }
    }
}

/// Display the [`Season`]s associated with a [`League`]
async fn get_seasons_for_league(
    State(state): State<AppState>,
    Path(league_id): Path<Uuid>,
) -> Result<Html<String>, LeagusError> {
    let store = &state.store;
    let league_id = LeagueId::from(league_id);
    let league = store.get_league(&league_id).await;
    let seasons = store.list_seasons_for_league(&league_id).await;

    match league {
        None => Err(LeagusError::Internal), // TODO: Return better error
        Some(league) => Ok(Html(
            SeasonsForLeagueTemplate {
                league,
                seasons,
                active_season: None,
            }
            .to_string(),
        )),
    }
}

/// Post the form for creating new venue
pub async fn post_create_league(
    State(state): State<AppState>,
    Form(input): Form<CreateLeagueInput>,
) -> Result<Html<String>, LeagusError> {
    if input.league_name.is_empty() {
        return Err(LeagusError::Internal);
    }

    let league = League::new(&input.league_name, "");
    let store = &state.store;
    info!("Creating league; {:?}", &league);
    store.create_league(league).await;

    let leagues = store.list_leagues().await;

    Ok(Html(
        LeaguesListTemplate {
            items: leagues,
            url_base: "leagues".to_string(),
        }
        .to_string(),
    ))
}

#[derive(Deserialize, Debug)]
pub struct CreateLeagueInput {
    #[serde(default)]
    league_name: String,
}

/// Get the target season to display
///
/// If a season_id is provided in the query params then that will become the target season,
/// otherwise if the league has an active season set that will become the target season.
fn get_target_season(
    seasons: &[Season],
    league: &League,
    params: Option<&Query<GetQueryParams>>,
) -> Option<Season> {
    let target_season_id = params
        .and_then(|p| p.season_id)
        .map(SeasonId::from)
        .or(league.active_season);

    seasons
        .iter()
        .find(|x| Some(x.id) == target_season_id)
        .cloned()
}

#[derive(Deserialize, Debug)]
struct GetQueryParams {
    season_id: Option<Uuid>,
}
