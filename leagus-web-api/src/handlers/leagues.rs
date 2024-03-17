use askama::Template;
use axum::extract::Path;
use axum::response::Html;
use axum::{routing::get, Router};
use axum_htmx::{HxBoosted, HxRequest};
use bson::Uuid;
use leagus::models::{League, Season, SeasonTable};
use leagus::persistence::mongo_store::MongoStore;
use leagus::persistence::WriteableStore;

/// Routes available for '/leagues' path.
pub fn routes() -> Router {
    // TODO: Error handling
    Router::new()
        .route("/", get(list))
        .route("/:league_id", get(get_by_id))
}

async fn list(HxBoosted(boosted): HxBoosted) -> Html<String> {
    // TODO: Don't create a new MongoStore every request
    let store = MongoStore::new().await.unwrap();
    let leagues = store.list_leagues().await;

    if boosted {
        Html(
            LeaguesPartialTemplate {
                title: "Leagues",
                leagues,
            }
            .to_string(),
        )
    } else {
        Html(
            LeaguesFullTemplate {
                title: "Leagues",
                headings: vec!["Leagues", "Players", "Tables"],
                leagues,
            }
            .to_string(),
        )
    }
}

async fn get_by_id(
    HxRequest(hxrequest): HxRequest,
    HxBoosted(boosted): HxBoosted,
    Path(league_id): Path<Uuid>,
) -> Html<String> {
    // TODO: Don't create a new MongoStore every request
    let store = MongoStore::new().await.unwrap();
    let league = store.get_league(&league_id).await;
    let seasons = store.list_seasons_for_league(&league_id).await;

    match league {
        Some(league) => {
            if boosted || hxrequest {
                Html(
                    LeagueContentTemplate {
                        league,
                        seasons,
                        season_table: SeasonTable::new(),
                    }
                    .to_string(),
                )
            } else {
                Html(
                    LeagueTemplate {
                        headings: vec!["Leagues", "Players", "Tables"],
                        league,
                        seasons,
                        season_table: SeasonTable::new(),
                    }
                    .to_string(),
                )
            }
        }
        None => Html("League not found".to_string()),
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
