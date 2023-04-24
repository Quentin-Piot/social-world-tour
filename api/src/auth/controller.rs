use crate::server::AppState;
use axum::extract::Query;
use axum::response::{IntoResponse, Redirect};
use axum::{extract::State, routing::get, Json, Router};
use chrono::Utc;
use oauth2::basic::BasicClient;
use oauth2::CsrfToken;

use crate::auth::models::Claims;
use crate::auth::service::generate_token_from_authorization_code;
use crate::error::AppError;
use serde::{Deserialize, Serialize};
use entity::users;

use social_world_tour_core::users::Query as QueryCore;
use social_world_tour_core::users::Mutation as MutationCore;


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
   state: State<AppState>,
    query: Query<CallbackQuery>,
) -> Result<Json<AuthBody>, AppError> {
    let callback_query: CallbackQuery = query.0;
    let token_response = generate_token_from_authorization_code(state.oauth_client.to_owned(), callback_query.code)
        .await?;
    let user_data = token_response.user_data;

    let user = QueryCore::find_user_by_email(&state.conn, &user_data.email).await.map_err(|_| AppError::InternalServerError)?;
    if user.is_none() {
        let given_name = match user_data.given_name {
            Some(name) => name,
            None=> user_data.email.to_owned()
        };
        let user_model = users::Model {
            email:user_data.email.to_owned(),
            username:given_name,
            created_at: Utc::now().naive_local(),
            ..Default::default()
        };

        let created_user = MutationCore::create_user(&state.conn, user_model).await;
        if created_user.is_err() {
            return Err(AppError::InternalServerError);
        }
    }


    Ok(Json(AuthBody::new(token_response.token)))
}
