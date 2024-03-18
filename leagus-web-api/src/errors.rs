use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum Result<T> {
    Ok(T),
    Err(LeagusError),
}

/// All errors exposed via external interfaces
pub enum LeagusError {
    /// Something unexpected went wrong
    Internal,
}

impl IntoResponse for LeagusError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            LeagusError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong"),
        };

        (status, message).into_response()
    }
}
