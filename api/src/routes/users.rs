use crate::AppState;
use axum::{Json, Router};

pub fn router() -> Router<AppState> {
    Router::new()
}
