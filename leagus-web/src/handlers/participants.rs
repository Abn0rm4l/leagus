use axum::{extract::State, response::Html, routing::get, Form, Router};
use leagus::{models::Participant, persistence::WriteableStore};
use serde::Deserialize;

use crate::{
    errors::LeagusError,
    state::AppState,
    templates::{CreateParticipantTemplate, ParticipantsListTemplate, ParticipantsTemplate},
};

/// Routes available for '/participants' path.
pub fn routes<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/", get(list))
        .route(
            "/create",
            get(get_create_participant).post(post_create_participant),
        )
        .with_state(state)
}

async fn list(State(state): State<AppState>) -> Result<Html<String>, LeagusError> {
    let store = &state.store;
    let participants = store.list_participants(None).await;
    Ok(Html(ParticipantsTemplate { participants }.to_string()))
}

/// Get the form for creating a new participant
pub async fn get_create_participant() -> Result<Html<String>, LeagusError> {
    Ok(Html(CreateParticipantTemplate {}.to_string()))
}

/// Post the form for createing new participant
pub async fn post_create_participant(
    State(state): State<AppState>,
    Form(input): Form<CreateParticipantInput>,
) -> Result<Html<String>, LeagusError> {
    if input.participant_name.is_empty() {
        return Err(LeagusError::Internal);
    }

    let participant = Participant::new(input.participant_name);
    let store = &state.store;
    store.create_participant(&participant).await;

    Ok(Html(
        ParticipantsListTemplate {
            items: store.list_participants(None).await,
            url_base: "participants".to_string(),
        }
        .to_string(),
    ))
}

#[derive(Deserialize, Debug)]
pub struct CreateParticipantInput {
    #[serde(default)]
    participant_name: String,
}
