use axum::{
    Extension,
    extract::{Path, State},
    Json,
    middleware, Router, routing::delete,
};
use axum::routing::get;

use social_world_tour_core::users::Mutation as MutationCore;
use social_world_tour_core::users::Query as QueryCore;

use crate::error::AppError;
use crate::server::AppState;
use crate::users::response::UserResponse;
use crate::users::service::validate_user;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users/me", get(get_me_handler))
        .route("/users/:id", delete(delete_user))
        .route_layer(middleware::from_fn(validate_user))
}

pub async fn get_me_handler(
    state: State<AppState>,
    Extension(email): Extension<String>,
) -> Result<Json<UserResponse>, AppError> {
    let result = QueryCore::find_user_by_email(&state.conn, &email)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    let user = result.ok_or(AppError::UserDoesNotExist)?;

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
        Err(_) => Err(AppError::InternalServerError),
    }
}
