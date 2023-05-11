use chrono::Utc;
use jsonwebtoken::{encode, Header};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, TokenResponse};

use entity::teams;
use entity::users;
use migration::sea_orm::DbConn;
use social_world_tour_core::teams::Mutation as TeamMutationCore;
use social_world_tour_core::user_teams::Mutation as UserTeamsMutationCore;
use social_world_tour_core::users::Mutation as UserMutationCore;

use crate::auth::models::{Claims, OAuthUser, KEYS};
use crate::error::AppError;

pub struct GenerateTokenResponse {
    pub token: String,
    pub user_data: OAuthUser,
}

pub async fn generate_token_from_authorization_code(
    oauth_client: BasicClient,
    authorization_code: String,
) -> Result<GenerateTokenResponse, AppError> {
    let token_result = oauth_client
        .exchange_code(AuthorizationCode::new(authorization_code))
        .request_async(async_http_client)
        .await
        .unwrap();
    let access_token = token_result.access_token().secret();

    let client = reqwest::Client::new();
    let user_data = client
        .get("https://dev-social-media-tour.eu.auth0.com/userinfo")
        .bearer_auth(access_token)
        .send()
        .await
        .unwrap()
        .json::<OAuthUser>()
        .await
        .unwrap();

    let date_in_one_week = Utc::now() + chrono::Duration::days(7);

    let claims = Claims {
        sub: user_data.email.to_owned(),
        auth0_sub: user_data.sub.to_owned(),
        exp: date_in_one_week.timestamp() as usize,
    };

    let token = encode(&Header::default(), &claims, &KEYS.encoding);
    if token.is_err() {
        return Err(AppError::InvalidToken);
    }
    let token_response = GenerateTokenResponse {
        token: token.unwrap(),
        user_data,
    };
    Ok(token_response)
}

pub async fn create_user_and_team(conn: &DbConn, user_data: OAuthUser) -> Result<(), AppError> {
    let given_name = match user_data.given_name {
        Some(name) => name,
        None => user_data.email.to_owned(),
    };
    let user_model = users::Model {
        id: 0,
        email: user_data.email.to_owned(),
        username: given_name,
        created_at: Utc::now().naive_local(),
    };

    let created_user_result = UserMutationCore::create_user(&conn, user_model)
        .await
        .map_err(|err| AppError::InternalServerError(Some(err.to_string())))?;

    let created_user_id = created_user_result.id;
    let team_model = teams::Model {
        id: 0,
        name: None,
        logo: None,
        created_by: created_user_id.to_owned(),
        created_at: Utc::now().naive_local(),
    };

    let created_team = TeamMutationCore::create_team(&conn, team_model)
        .await
        .map_err(|err| AppError::InternalServerError(Some(err.to_string())))?;

    UserTeamsMutationCore::create_user_teams(&conn, created_team.id, created_user_id)
        .await
        .map_err(|err| AppError::InternalServerError(Some(err.to_string())))?;

    Ok(())
}
