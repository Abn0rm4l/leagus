use axum::{
    extract::{Path, State},
    response::Html,
    routing::{get, post},
    Router,
};
use bson::Uuid;
use chrono::Utc;
use leagus::{
    models::{SeasonId, Session},
    persistence::WriteableStore,
};

use crate::{errors::LeagusError, state::AppState};

type LeagusResult = Result<Html<String>, LeagusError>;

/// Routes available for '/sessions' path.
pub fn routes<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/", get(list))
        .route("/create/:league_id", post(post_create_session))
        .with_state(state)
}

pub async fn list(State(state): State<AppState>) -> LeagusResult {
    let store = &state.store;
    let sessions = store.list_sessions().await;
    Ok(Html(serde_json::to_string(&sessions).unwrap()))
}

pub async fn post_create_session(
    State(state): State<AppState>,
    Path(season_id): Path<Uuid>,
) -> LeagusResult {
    let store = &state.store;
    let season_id = SeasonId::from(season_id);
    let season = store.get_season(&season_id).await;
    let new_session = Session::new(&season_id, &Utc::now());

    // Make sure the season exists before trying to add a new session
    match season {
        // TODO: Add better error
        None => Err(LeagusError::Internal),
        Some(_) => {
            store.create_session(&new_session).await;
            println!("Creating session: {:?}", new_session);
            //TODO: create a better response
            Ok(Html("Session Created".to_string()))
        }
    }
}
