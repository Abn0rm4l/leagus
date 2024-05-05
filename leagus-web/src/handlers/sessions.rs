use axum::{
    extract::{Path, State},
    response::Html,
    routing::{get, post},
    Router,
};
use bson::Uuid;
use chrono::Utc;
use futures::join;
use leagus::{
    models::{SeasonId, Session, SessionId},
    persistence::WriteableStore,
};

use crate::{
    errors::LeagusError,
    state::AppState,
    templates::{RoundViewTemplate, SessionViewTemplate},
};

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
            tracing::info!("Created session: {:?}", new_session);
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
    let session = store.get_session(&session_id);
    let rounds = store.list_rounds_for_session(&session_id);
    // Fetch both together
    let (session, rounds) = join!(session, rounds);
    let active_round = rounds.last().cloned();

    if session.is_none() {
        return Err(LeagusError::Internal);
    }
    let session = session.expect("Session should exist");

    //TODO: Use the RoundViewTemplate
    let participants = match &active_round {
        Some(round) => store.list_participants_for_round(&round.id).await,
        None => Vec::new(),
    };

    let round_view_template = RoundViewTemplate {
        session: session.clone(),
        active_round,
        rounds,
        participants,
        update_participants_template: None,
    };

    Ok(Html(
        SessionViewTemplate {
            session,
            round_view_template,
        }
        .to_string(),
    ))
}
