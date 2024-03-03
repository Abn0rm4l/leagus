use axum::Router;
use handlers::{api, leagues, matches, root, seasons, sessions, venues};
use tower_http::services::ServeDir;

mod handlers;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/", root::routes())
        .nest("/api", api::routes())
        .nest("/leagues", leagues::routes())
        .nest("/seasons", seasons::routes())
        .nest("/sessions", sessions::routes())
        .nest("/matches", matches::routes())
        .nest("/venues", venues::routes())
        .nest_service("/assets", ServeDir::new("assets"));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
