use axum::{
    extract::{Path, State},
    routing::{post,delete},
    Json, Router,
};
use chrono::Utc;

use crate::users::dto::{CreateUserInput, UpdateUserInput};
use crate::users::errors::UserError;

use crate::server::AppState;
use entity::users;
use social_world_tour_core::sea_orm::DbErr;
use social_world_tour_core::users::Mutation as MutationCore;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/:id", delete(delete_user))
}

async fn create_user(
    state: State<AppState>,
    body: Json<CreateUserInput>,
) -> Result<Json<String>, UserError> {
    let create_user_input = body.0;

    let user_model = users::Model {
        username: create_user_input.username,
        email: create_user_input.email,
        created_at: Utc::now().naive_local(),
        ..Default::default()
    };

    let result = MutationCore::create_user(&state.conn, user_model).await;
    match result {
        Ok(_) => Ok(Json("User created".to_owned())),
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
