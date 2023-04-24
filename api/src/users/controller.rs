use axum::{
    extract::{Path, State},
    routing::{delete},
    Json, Router,
};

use crate::users::errors::UserError;

use crate::server::AppState;
use social_world_tour_core::users::Mutation as MutationCore;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users/:id", delete(delete_user))
}

async fn delete_user(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<String>, UserError> {
    let result = MutationCore::delete_user(&state.conn, id).await;
    match result {
        Ok(_) => Ok(Json("User successfully deleted".to_owned())),
        Err(_) => Err(UserError::InternalServerError),
    }
}
