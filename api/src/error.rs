use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    InvalidToken,
    WrongCredential,
    MissingCredential,
    TokenCreation,
    InternalServerError(Option<String>),
    UserDoesNotExist,
    UserAlreadyExits,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalServerError(custom_msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!(
                    "Internal server error: {}",
                    custom_msg.unwrap_or("".to_string())
                ),
            ),
            Self::InvalidToken => (StatusCode::BAD_REQUEST, format!("Invalid token")),
            Self::MissingCredential => (StatusCode::BAD_REQUEST, format!("Missing credential")),
            Self::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create token"),
            ),
            Self::WrongCredential => (StatusCode::UNAUTHORIZED, format!("Wrong credentials")),
            Self::UserDoesNotExist => (StatusCode::UNAUTHORIZED, format!("User does not exist")),
            Self::UserAlreadyExits => (StatusCode::BAD_REQUEST, format!("User already exists")),
        };
        (status, Json(json!({ "Error": err_msg }))).into_response()
    }
}
