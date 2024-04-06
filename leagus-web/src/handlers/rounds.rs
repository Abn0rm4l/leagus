use axum::{
    extract::{Path, State},
    response::Html,
    routing::{get, post},
    Router,
};
use bson::Uuid;
use leagus::{
    models::{Participant, ParticipantId, Round, RoundId, SessionId},
    persistence::WriteableStore,
};

use crate::{
    errors::LeagusError,
    state::AppState,
    templates::{RoundViewTemplate, UpdateRoundParticipantsTemplate},
};

type LeagusResult = Result<Html<String>, LeagusError>;

/// Routes available for '/rounds' path.
pub fn routes<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/:round_id", get(get_round))
        .route(
            "/:round_id/update_participants",
            get(get_update_participants),
        )
        .route("/create/:session_id", post(create_round))
        .with_state(state)
}

pub async fn get_round(State(state): State<AppState>, Path(round_id): Path<Uuid>) -> LeagusResult {
    let store = &state.store;
    let round_id = RoundId::from(round_id);
    let round = store.get_round(&round_id).await;

    //TODO: Replace with real data
    let mut fake_participants = vec![
        Participant {
            id: ParticipantId::new(),
            name: "Minnie".to_string(),
        },
        Participant {
            id: ParticipantId::new(),
            name: "Johnathan".to_string(),
        },
        Participant {
            id: ParticipantId::new(),
            name: "Charles".to_string(),
        },
        Participant {
            id: ParticipantId::new(),
            name: "Jess".to_string(),
        },
    ];

    let mut participants = store.list_participants_for_round(&round_id).await;

    // TODO: Remove when done testing
    participants.append(&mut fake_participants);

    match round {
        // TODO: Add better error, e.g. not found
        None => Err(LeagusError::Internal),
        Some(round) => {
            let rounds = store.list_rounds_for_session(&round.session_id).await;
            let session = store.get_session(&round.session_id).await;

            if session.is_none() {
                return Err(LeagusError::Internal);
            }

            Ok(Html(
                RoundViewTemplate {
                    session: session.expect("Session linked to round is missing"),
                    rounds,
                    active_round: Some(round),
                    participants,
                }
                .to_string(),
            ))
        }
    }
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
                    participants: Vec::new(),
                }
                .to_string(),
            ))
        }
    }
}

pub async fn get_update_participants(
    State(state): State<AppState>,
    Path(round_id): Path<Uuid>,
) -> LeagusResult {
    let store = &state.store;
    let round_id = RoundId::from(round_id);
    let round = store.get_round(&round_id).await;

    if round.is_none() {
        // TODO: Return bad argument error
        return Err(LeagusError::Internal);
    }

    // fetch all participants
    let participants = store.list_participants().await;

    Ok(Html(
        UpdateRoundParticipantsTemplate {
            participants,
            round_id,
        }
        .to_string(),
    ))
}
