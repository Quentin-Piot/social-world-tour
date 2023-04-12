use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use chrono::Utc;
use serde_json::{json, Value};

use crate::error::AppError;
use crate::server::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_checker))
        .route("/healthchecker", get(health_checker))
}

async fn health_checker(state: State<AppState>) -> Result<Json<Value>, AppError> {
    const MESSAGE: &str = "How to Implement Google OAuth2 in Rust";

    Ok(Json(json!({"status": "success", "message": MESSAGE})))
}
