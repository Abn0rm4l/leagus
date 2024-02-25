use axum::Router;
use handlers::{leagues, matches, seasons, sessions, venues};

mod handlers;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/leagues", leagues::routes())
        .nest("/seasons", seasons::routes())
        .nest("/sessions", sessions::routes())
        .nest("/matches", matches::routes())
        .nest("/venues", venues::routes());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
