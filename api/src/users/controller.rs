use axum::routing::get;
use axum::{
    extract::{Path, State},
    middleware,
    routing::delete,
    Extension, Json, Router,
};

use entity::users::Model;
use social_world_tour_core::users::Mutation as MutationCore;

use crate::error::AppError;
use crate::server::AppState;
use crate::users::response::UserResponse;
use crate::users::service::validate_user;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/users/me", get(get_me_handler))
        .route("/users/:id", delete(delete_user))
        .route_layer(middleware::from_fn_with_state(state.clone(), validate_user))
        .with_state(state)
}

pub async fn get_me_handler(
    Extension(user): Extension<Model>,
) -> Result<Json<UserResponse>, AppError> {
    let json_response = UserResponse {
        email: user.email,
        username: user.username,
    };

    Ok(Json(json_response))
}

async fn delete_user(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<String>, AppError> {
    let result = MutationCore::delete_user(&state.conn, id).await;
    match result {
        Ok(_) => Ok(Json("User successfully deleted".to_owned())),
        Err(err) => Err(AppError::InternalServerError(Some(err.to_string()))),
    }
}
