use axum::routing::{get, post};
use axum::{extract::State, middleware, Extension, Json, Router};
use chrono::Utc;

use entity::teams::Model;
use entity::users;
use social_world_tour_core::teams::Mutation as MutationCore;
use social_world_tour_core::teams::Query as QueryCore;

use crate::error::AppError;
use crate::server::AppState;
use crate::teams::dto::CreateTeamInput;
use crate::teams::response::{TeamResponse, TeamsResponse};
use crate::users::service::validate_user;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/teams/me", get(get_my_teams))
        .route("/teams", post(create_team))
        .route_layer(middleware::from_fn_with_state(state.clone(), validate_user))
        .with_state(state)
}

async fn create_team(
    Extension(user): Extension<users::Model>,
    state: State<AppState>,
    Json(payload): Json<CreateTeamInput>,
) -> Result<Json<TeamResponse>, AppError> {
    let team_input = Model {
        id: 0,
        name: Some(payload.name),
        logo: payload.logo,
        created_by: user.id,
        created_at: Utc::now().naive_local(),
    };

    let team = MutationCore::create_team(&state.conn, team_input)
        .await
        .map_err(|err| AppError::InternalServerError(Some(err.to_string())))?;

    let team_response = TeamResponse {
        name: team.name.to_owned(),
        logo: team.logo,
    };
    Ok(Json(team_response))
}

pub async fn get_my_teams(
    state: State<AppState>,
    Extension(user): Extension<users::Model>,
) -> Result<Json<TeamsResponse>, AppError> {
    let teams_result = QueryCore::find_teams_by_user_id(&state.conn, user.id)
        .await
        .map_err(|err| AppError::InternalServerError(Some(err.to_string())))?;
    let teams_response: Vec<TeamResponse> = teams_result
        .iter()
        .map(|team| TeamResponse {
            name: team.to_owned().name,
            logo: team.to_owned().logo,
        })
        .collect();

    Ok(Json(TeamsResponse {
        teams: teams_response,
    }))
}
