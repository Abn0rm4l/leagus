use axum::{
    extract::{Path, State},
    response::Html,
    routing::{get, post},
    Router,
};
use bson::Uuid;
use chrono::Utc;
use leagus::{
    models::{SeasonId, Session, SessionId},
    persistence::WriteableStore,
};

use crate::{errors::LeagusError, state::AppState, templates::SessionViewTemplate};

type LeagusResult = Result<Html<String>, LeagusError>;

/// Routes available for '/sessions' path.
pub fn routes<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/", get(list))
        .route("/:session_id", get(get_session))
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

pub async fn get_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> LeagusResult {
    let store = &state.store;
    let session_id = SessionId::from(session_id);
    let session = store.get_session(&session_id).await;

    match session {
        // TODO: Add better error, e.g. not found
        None => Err(LeagusError::Internal),
        Some(session) => Ok(Html(SessionViewTemplate { session }.to_string())),
    }
}
