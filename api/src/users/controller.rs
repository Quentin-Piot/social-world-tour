use std::sync::Arc;

use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::middleware::Next;
use axum::response::{ErrorResponse, IntoResponse, Response};
use axum::routing::get;
use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    middleware,
    routing::delete,
    Extension, Json, Router, TypedHeader,
};
use axum_extra::extract::cookie::CookieJar;
use http::Request;
use jsonwebtoken::{decode, Validation};
use serde_json::json;

use entity::prelude::Users;
use social_world_tour_core::users::Mutation as MutationCore;
use social_world_tour_core::users::Query as QueryCore;

use crate::auth::models::{Claims, KEYS};
use crate::error::AppError;
use crate::server::AppState;
use crate::users::dto::UserResponse;
use crate::users::errors::UserError;
use crate::users::service::get_user_email_from_token;
use entity::users::Model as UserModel;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/users/me", get(get_me_handler))
        .route_layer(middleware::from_fn_with_state(state, auth))
        .route("/users/:id", delete(delete_user))
}

pub async fn get_me_handler(
    Extension(user): Extension<UserModel>,
) -> Result<Json<UserResponse>, AppError> {
    let json_response = UserResponse {
        email: user.email,
        username: user.username,
    };

    Ok(Json(json_response))
}

pub async fn auth<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    state: State<AppState>,
    mut request: Request<B>,
    next: Next<B>,
) -> Response {
    let token = auth.token();
    println!("token: {}", token);

    let claims = decode::<Claims>(&token, &KEYS.decoding, &Validation::default())
        .unwrap()
        .claims;

    let user_email = claims.sub;
    println!("user_email: {}", user_email);

    let user = QueryCore::find_user_by_email(&state.conn, &user_email)
        .await
        .unwrap()
        .unwrap();

    request.extensions_mut().insert(user);

    let response = next.run(request).await;
    response
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
