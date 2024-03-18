use askama::Template;
use axum::extract::{Path, State};
use axum::response::Html;
use axum::{routing::get, Router};
use axum_htmx::{HxBoosted, HxRequest};
use bson::Uuid;
use leagus::models::{League, Season, SeasonTable};
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

async fn list(
    State(state): State<AppState>,
    HxBoosted(boosted): HxBoosted,
) -> Result<Html<String>, LeagusError> {
    let store = &state.store;
    let leagues = store.list_leagues().await;

    if boosted {
        Ok(Html(
            LeaguesPartialTemplate {
                title: "Leagues",
                leagues,
            }
            .to_string(),
        ))
    } else {
        Ok(Html(
            LeaguesFullTemplate {
                title: "Leagues",
                headings: vec!["Leagues", "Players", "Tables"],
                leagues,
            }
            .to_string(),
        ))
    }
}

async fn get_by_id(
    State(state): State<AppState>,
    HxRequest(hxrequest): HxRequest,
    HxBoosted(boosted): HxBoosted,
    Path(league_id): Path<Uuid>,
) -> Result<Html<String>, LeagusError> {
    let store = &state.store;
    let league = store.get_league(&league_id).await;
    let seasons = store.list_seasons_for_league(&league_id).await;

    match league {
        Some(league) => {
            if boosted || hxrequest {
                Ok(Html(
                    LeagueContentTemplate {
                        league,
                        seasons,
                        season_table: SeasonTable::new(),
                    }
                    .to_string(),
                ))
            } else {
                Ok(Html(
                    LeagueTemplate {
                        headings: vec!["Leagues", "Players", "Tables"],
                        league,
                        seasons,
                        season_table: SeasonTable::new(),
                    }
                    .to_string(),
                ))
            }
        }
        None => Err(LeagusError::Internal), // TODO: Return better error
    }
}

// -- Templates

#[derive(Template)]
#[template(path = "leagues.html")]
struct LeaguesFullTemplate<'a> {
    title: &'a str,
    headings: Vec<&'a str>,
    leagues: Vec<League>,
}

#[derive(Template)]
#[template(path = "partials/leagues_content.html")]
struct LeaguesPartialTemplate<'a> {
    title: &'a str,
    leagues: Vec<League>,
}

#[derive(Template)]
#[template(path = "partials/leagues/single_content.html")]
struct LeagueContentTemplate {
    league: League,
    seasons: Vec<Season>,
    season_table: SeasonTable,
}

#[derive(Template)]
#[template(path = "league.html")]
struct LeagueTemplate<'a> {
    headings: Vec<&'a str>,
    league: League,
    seasons: Vec<Season>,
    season_table: SeasonTable,
}
