use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::TypedHeader;
use http::Request;
use jsonwebtoken::{decode, Validation};

use crate::auth::models::{Claims, KEYS};
use crate::error::AppError;

pub async fn validate_user<B>(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, AppError> {
    let token = auth.token();

    let claims = decode::<Claims>(token, &KEYS.decoding, &Validation::default())
        .map_err(|_| AppError::WrongCredential)?
        .claims;

    let user_email = claims.sub;

    request.extensions_mut().insert(user_email);

    Ok(next.run(request).await)
}
