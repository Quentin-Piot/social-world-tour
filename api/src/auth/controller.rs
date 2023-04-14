use crate::server::AppState;
use axum::extract::Query;
use axum::response::{IntoResponse, Redirect};
use axum::{extract::State, routing::get, Json, Router};
use oauth2::basic::BasicClient;
use oauth2::CsrfToken;

use crate::auth::models::Claims;
use crate::auth::service::generate_token_from_authorization_code;
use crate::error::AppError;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CallbackQuery {
    code: String,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth", get(authorize))
        .route("/auth/authorized", get(callback))
        .route("/protected", get(protected))
}

async fn protected(claims: Claims) -> Result<String, AppError> {
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{}",
        claims
    ))
}
async fn authorize(State(oauth_client): State<BasicClient>) -> impl IntoResponse {
    let (authorize_url, _csrf_state) = oauth_client.authorize_url(CsrfToken::new_random).url();
    Redirect::to(authorize_url.as_ref())
}

async fn callback(
    State(oauth_client): State<BasicClient>,
    query: Query<CallbackQuery>,
) -> Result<Json<AuthBody>, AppError> {
    let callback_query: CallbackQuery = query.0;
    let token = generate_token_from_authorization_code(oauth_client, callback_query.code)
        .await
        .map_err(|_| AppError::TokenCreation)?;

    Ok(Json(AuthBody::new(token)))
}
