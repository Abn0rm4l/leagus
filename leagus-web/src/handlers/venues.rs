use askama::Template;
use axum::{extract::State, response::Html, routing::get, Form, Router};
use leagus::{models::Venue, persistence::WriteableStore};
use serde::Deserialize;

use crate::{errors::LeagusError, state::AppState};

/// Routes available for '/venues' path.
pub fn routes<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/", get(list))
        .route("/create", get(get_create_venue).post(post_create_venue))
        .with_state(state)
}

async fn list(State(state): State<AppState>) -> Result<Html<String>, LeagusError> {
    let store = &state.store;
    let venues = store.list_venues().await;
    Ok(Html(VenuesTemplate { venues }.to_string()))
}

/// Get the form for creating a new venue
pub async fn get_create_venue() -> Result<Html<String>, LeagusError> {
    Ok(Html(CreateVenueTemplate {}.to_string()))
}

/// Post the form for createing new venue
pub async fn post_create_venue(
    State(state): State<AppState>,
    Form(input): Form<CreateVenueInput>,
) -> Result<Html<String>, LeagusError> {
    if input.venue_name.is_empty() {
        return Err(LeagusError::Internal);
    }

    let venue = Venue::new(input.venue_name);
    let store = &state.store;
    store.create_venue(&venue).await;

    Ok(Html("venue Created!".to_string()))
}

// -- Templates

#[derive(Template)]
#[template(path = "venues.html")]
struct VenuesTemplate {
    venues: Vec<Venue>,
}

#[derive(Template)]
#[template(path = "partials/venues/create_venue.html")]
struct CreateVenueTemplate {}

#[derive(Deserialize, Debug)]
pub struct CreateVenueInput {
    #[serde(default)]
    venue_name: String,
}
