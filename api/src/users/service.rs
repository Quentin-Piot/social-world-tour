use axum::extract::State;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::TypedHeader;
use http::Request;
use jsonwebtoken::{decode, Validation};

use social_world_tour_core::users::Query as QueryCore;

use crate::auth::models::{Claims, KEYS};
use crate::error::AppError;
use crate::server::AppState;

pub async fn validate_user<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, AppError> {
    let token = auth.token();

    let claims = decode::<Claims>(token, &KEYS.decoding, &Validation::default())
        .map_err(|_| AppError::WrongCredential)?
        .claims;

    let user_email = claims.sub;

    let user = QueryCore::find_user_by_email(&state.conn, &user_email)
        .await
        .map_err(|err| AppError::InternalServerError(Some(err.to_string())))?
        .ok_or(AppError::UserDoesNotExist)?;

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
