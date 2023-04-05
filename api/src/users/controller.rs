use axum::{
    extract::{Form, Path, State},
    routing::delete,
    routing::post,
    Json, Router,
};
use chrono::Utc;

use crate::users::dto::{CreateUserInput, UpdateUserInput};
use crate::users::errors::UserError;
use crate::AppState;
use entity::users;
use social_world_tour_core::users::Mutation as MutationCore;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/:id", post(update_user).delete(delete_user))
}

async fn create_user(
    state: State<AppState>,
    body: Json<CreateUserInput>,
) -> Result<Json<String>, UserError> {
    let create_user_input = body.0;

    let user_model = users::PartialModel {
        username: Some(create_user_input.username),
        email: Some(create_user_input.email),
        created_at: Some(Utc::now().naive_local()),
        ..Default::default()
    };

    let result = MutationCore::create_user(&state.conn, user_model).await;
    match result {
        Ok(_) => Ok(Json("User created".to_owned())),
        Err(_) => Err(UserError::InternalServerError),
    }
}

async fn update_user(
    state: State<AppState>,
    Path(id): Path<i32>,
    body: Json<UpdateUserInput>,
) -> Result<Json<String>, UserError> {
    let update_user_input = body.0;
    let user_model = users::PartialModel {
        username: update_user_input.username,
        ..Default::default()
    };

    let result = MutationCore::update_user_by_id(&state.conn, id, user_model).await;
    match result {
        Ok(_) => Ok(Json("User successfully updated".to_owned())),
        Err(_) => Err(UserError::InternalServerError),
    }
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
