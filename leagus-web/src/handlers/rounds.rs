use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::{get, post},
    Router,
};
use bson::Uuid;
use futures::join;
use leagus::{
    models::{Participant, ParticipantId, Round, RoundId, SessionId},
    persistence::WriteableStore,
};
use serde::Deserialize;

use crate::{
    errors::LeagusError,
    state::AppState,
    templates::{RoundParticipantsTemplate, RoundViewTemplate, UpdateRoundParticipantsTemplate},
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
        .route(
            "/:round_id/add_participant/:participant_id",
            post(add_participant_to_round),
        )
        .route("/create/:session_id", post(create_round))
        .with_state(state)
}

pub async fn get_round(State(state): State<AppState>, Path(round_id): Path<Uuid>) -> LeagusResult {
    let store = &state.store;
    let round_id = RoundId::from(round_id);
    let round = store.get_round(&round_id).await;
    let participants = store.list_participants_for_round(&round_id).await;

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
                    update_participants_template: None,
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
                    update_participants_template: None,
                }
                .to_string(),
            ))
        }
    }
}

pub async fn get_update_participants(
    State(state): State<AppState>,
    Path(round_id): Path<Uuid>,
    params: Option<Query<SearchParticipantsQueryParams>>,
) -> LeagusResult {
    let store = &state.store;
    let round_id = RoundId::from(round_id);
    let round = store.get_round(&round_id).await;

    if round.is_none() {
        // TODO: Return bad argument error
        return Err(LeagusError::Internal);
    }

    // fetch all participants
    let all_participants = params
        .and_then(|p| p.query_name.clone())
        .map(|name| store.list_participants(Some(name)))
        .unwrap_or(store.list_participants(None));

    // None => store.list_participants(None),
    // fetch participants already added to the round
    let round_participants = store.list_participants_for_round(&round_id);
    let (all_participants, round_participants) = join!(all_participants, round_participants);

    // Remove participants already added to the round
    let participants = all_participants
        .into_iter()
        .filter(|x| !round_participants.contains(x))
        .collect();

    Ok(Html(
        UpdateRoundParticipantsTemplate {
            participants,
            round_id,
        }
        .to_string(),
    ))
}

pub async fn add_participant_to_round(
    State(state): State<AppState>,
    Path((round_id, participant_id)): Path<(Uuid, Uuid)>,
) -> LeagusResult {
    let store = &state.store;
    let participant_id = ParticipantId::from(participant_id);
    let round_id = RoundId::from(round_id);
    let round = store.get_round(&round_id);
    let participant = store.get_participant(&participant_id);
    let (round, participant) = join!(round, participant);

    if round.is_none() || participant.is_none() {
        // TODO: Return bad argument error
        return Err(LeagusError::Internal);
    }
    let round = round.expect("Round now exists.");

    // add participant to the round
    store
        .add_participant_to_round(&participant_id, &round_id)
        .await;

    // fetch all participants
    let all_participants = store.list_participants(None);
    // fetch participants already added to the round
    let round_participants = store.list_participants_for_round(&round_id);
    let (all_participants, round_participants) = join!(all_participants, round_participants);

    // Remove participants already added to the round
    let participants: Vec<Participant> = all_participants
        .into_iter()
        .filter(|x| !round_participants.contains(x))
        .collect();

    let update_participants_template = Some(UpdateRoundParticipantsTemplate {
        participants,
        round_id,
    });

    Ok(Html(
        RoundParticipantsTemplate {
            active_round: round,
            round_id,
            participants: round_participants,
            update_participants_template,
        }
        .to_string(),
    ))
}

#[derive(Deserialize, Debug)]
pub struct SearchParticipantsQueryParams {
    query_name: Option<String>,
}
