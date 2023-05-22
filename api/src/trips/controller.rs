use axum::routing::{get, post};
use axum::{extract::State, middleware, Extension, Json, Router};
use chrono::Utc;

use entity::trips::Model;
use entity::users;
use social_world_tour_core::trips::Mutation as MutationCore;
use social_world_tour_core::trips::Query as QueryCore;

use crate::error::AppError;
use crate::server::AppState;
use crate::trips::dto::CreateTripInput;
use crate::trips::response::{TripResponse, TripsResponse};
use crate::users::service::validate_user;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/trips/me", get(get_my_trips))
        .route("/trips", post(create_trip))
        .route_layer(middleware::from_fn_with_state(state.clone(), validate_user))
        .with_state(state)
}

async fn create_trip(
    Extension(user): Extension<users::Model>,
    state: State<AppState>,
    Json(payload): Json<CreateTripInput>,
) -> Result<Json<TripResponse>, AppError> {
    let trip_input = Model {
        id: 0,
        name: Some(payload.name),
        logo: payload.logo,
        created_by: user.id,
        created_at: Utc::now().naive_local(),
    };

    let trip = MutationCore::create_trip(&state.conn, trip_input)
        .await
        .map_err(|err| AppError::InternalServerError(Some(err.to_string())))?;

    let trip_response = TripResponse {
        id: trip.id,
        name: trip.name.to_owned(),
        logo: trip.logo,
    };
    Ok(Json(trip_response))
}

pub async fn get_my_trips(
    state: State<AppState>,
    Extension(user): Extension<users::Model>,
) -> Result<Json<TripsResponse>, AppError> {
    let trips_result = QueryCore::find_trips_by_user_id(&state.conn, user.id)
        .await
        .map_err(|err| AppError::InternalServerError(Some(err.to_string())))?;
    let trips_response: Vec<TripResponse> = trips_result
        .iter()
        .map(|trip| TripResponse {
            id: trip.to_owned().id,
            name: trip.to_owned().name,
            logo: trip.to_owned().logo,
        })
        .collect();

    Ok(Json(TripsResponse {
        trips: trips_response,
    }))
}
