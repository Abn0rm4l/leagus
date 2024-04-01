use axum::{
    extract::{Path, State},
    response::Html,
    routing::post,
    Router,
};
use bson::Uuid;
use leagus::{
    models::{Round, SessionId},
    persistence::WriteableStore,
};

use crate::{errors::LeagusError, state::AppState, templates::RoundViewTemplate};

type LeagusResult = Result<Html<String>, LeagusError>;

/// Routes available for '/rounds' path.
pub fn routes<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/create/:session_id", post(create_round))
        .with_state(state)
}

pub async fn create_round(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> LeagusResult {
    let store = &state.store;
    let session_id = SessionId::from(session_id);
    let session = store.get_session(&session_id).await;

    match session {
        // TODO: Add better error, e.g. not found
        None => Err(LeagusError::Internal),
        Some(session) => {
            // Don't create a new round if the session_id does not exist
            let round = Round::new(session_id);
            store.create_round(&round).await;
            // Wait for creation to finish before fetching the list
            let rounds = store.list_rounds_for_session(&session_id).await;

            Ok(Html(
                RoundViewTemplate {
                    session,
                    rounds,
                    active_round: Some(round),
                }
                .to_string(),
            ))
        }
    }
}
