use std::env;

use axum::extract::Query;
use axum::response::Redirect;
use axum::{extract::State, routing::get, Json, Router};
use chrono::Utc;
use oauth2::basic::BasicClient;
use oauth2::CsrfToken;
use serde::Deserialize;

use entity::users;
use social_world_tour_core::users::Mutation as MutationCore;
use social_world_tour_core::users::Query as QueryCore;

use crate::auth::response::AuthorizeResponse;
use crate::auth::service::generate_token_from_authorization_code;
use crate::error::AppError;
use crate::server::AppState;

#[derive(Deserialize)]
struct CallbackQuery {
    code: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth", get(authorize))
        .route("/auth/authorized", get(callback))
}

async fn authorize(
    State(oauth_client): State<BasicClient>,
) -> Result<Json<AuthorizeResponse>, AppError> {
    let (authorize_url, _csrf_state) = oauth_client.authorize_url(CsrfToken::new_random).url();
    let response = AuthorizeResponse {
        callback_url: authorize_url.to_string(),
    };
    Ok(Json(response))
}

fn redirect_with_error(error: AppError) -> Redirect {
    let frontend_url = env::var("FRONTEND_URL").expect("FRONTEND_URL is not set in .env file");
    Redirect::to(format!("{}/auth/callback?error={:?}", frontend_url, error).as_str())
}

async fn callback(state: State<AppState>, query: Query<CallbackQuery>) -> Redirect {
    let frontend_url = env::var("FRONTEND_URL").expect("FRONTEND_URL is not set in .env file");

    let callback_query: CallbackQuery = query.0;
    let token_response =
        generate_token_from_authorization_code(state.oauth_client.to_owned(), callback_query.code)
            .await
            .expect("Error generating token from authorization code");

    let user_data = token_response.user_data;

    let res = QueryCore::find_user_by_email(&state.conn, &user_data.email).await;

    if res.is_err() {
        return redirect_with_error(AppError::InternalServerError);
    }

    let user = res.unwrap();

    if user.is_none() {
        let given_name = match user_data.given_name {
            Some(name) => name,
            None => user_data.email.to_owned(),
        };
        let user_model = users::Model {
            email: user_data.email.to_owned(),
            username: given_name,
            created_at: Utc::now().naive_local(),
            ..Default::default()
        };

        let created_user = MutationCore::create_user(&state.conn, user_model).await;
        if created_user.is_err() {
            return redirect_with_error(AppError::InternalServerError);
        }
    }

    Redirect::to(
        format!(
            "{}/auth/callback?token={}",
            frontend_url,
            token_response.token.as_str()
        )
        .as_str(),
    )
}
