use crate::auth::models::{Claims, OAuthUser, KEYS};
use axum::Json;
use http::StatusCode;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::users::errors::UserError;

pub fn get_user_email_from_token(token: String) -> String {
    let decoded = decode::<Claims>(&token, &KEYS.decoding, &Validation::default()).unwrap();

    return decoded.claims.sub;
}
