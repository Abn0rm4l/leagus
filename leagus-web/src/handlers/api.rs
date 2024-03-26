use axum::Router;

pub mod leagues;
pub mod matches;
pub mod seasons;
pub mod sessions;
pub mod venues;

/// Routes available for '/api' path.
pub fn routes() -> Router {
    // TODO: Error handling
    Router::new()
        .nest("/leagues", leagues::routes())
        .nest("/seasons", seasons::routes())
        .nest("/sessions", sessions::routes())
        .nest("/matches", matches::routes())
        .nest("/venues", venues::routes())
}
