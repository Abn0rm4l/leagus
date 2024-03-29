use axum::Router;
use handlers::{api, leagues, matches, participants, root, seasons, sessions, venues};
use state::AppState;
use tower_http::services::ServeDir;

mod errors;
mod handlers;
mod state;

#[tokio::main]
async fn main() {
    let state = AppState::new().await;

    let app = Router::new()
        .nest("/", root::routes())
        .nest("/api", api::routes())
        .nest("/leagues", leagues::routes(state.clone()))
        .nest("/seasons", seasons::routes(state.clone()))
        .nest("/sessions", sessions::routes(state.clone()))
        .nest("/matches", matches::routes())
        .nest("/venues", venues::routes(state.clone()))
        .nest("/participants", participants::routes(state.clone()))
        .nest_service("/assets", ServeDir::new("assets"));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
