use axum::extract::Path;
use axum::routing::{get, post};
use axum::{extract::State, middleware, Extension, Json, Router};
use chrono::Utc;

use entity::nodes::Model;
use entity::users;
use social_world_tour_core::nodes::Mutation as MutationCore;
use social_world_tour_core::nodes::Query as QueryCore;

use crate::error::AppError;
use crate::nodes::dto::CreateNodeInput;
use crate::nodes::response::{NodeResponse, NodesResponse};
use crate::server::AppState;
use crate::users::service::validate_user;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/nodes", post(create_node))
        .route("/nodes/trip/:id", get(get_nodes_for_trip))
        .route_layer(middleware::from_fn_with_state(state.clone(), validate_user))
        .with_state(state)
}

async fn create_node(
    Extension(user): Extension<users::Model>,
    state: State<AppState>,
    Json(payload): Json<CreateNodeInput>,
) -> Result<Json<NodeResponse>, AppError> {
    let node_input = Model {
        id: 0,
        country_code: payload.country_code,
        city: payload.city,
        title: payload.title,
        description: payload.description,
        latitude: payload.latitude,
        longitude: payload.longitude,
        trip: payload.trip,
        created_by: user.id,
        created_at: Utc::now().naive_local(),
    };

    let node = MutationCore::create_node(&state.conn, node_input)
        .await
        .map_err(|err| AppError::InternalServerError(Some(err.to_string())))?;

    let node_response = NodeResponse {
        id: node.id.to_owned(),
        country_code: node.country_code.to_owned(),
        city: node.city.to_owned(),
        title: node.title.to_owned(),
        description: node.description.to_owned(),
        latitude: node.latitude.to_owned(),
        longitude: node.longitude.to_owned(),
    };
    Ok(Json(node_response))
}

pub async fn get_nodes_for_trip(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<NodesResponse>, AppError> {
    let nodes_result = QueryCore::find_nodes_by_trip_id(&state.conn, id)
        .await
        .map_err(|err| AppError::InternalServerError(Some(err.to_string())))?;
    let nodes_response: Vec<NodeResponse> = nodes_result
        .iter()
        .map(|node| NodeResponse {
            id: node.id.to_owned(),
            country_code: node.country_code.to_owned(),
            city: node.city.to_owned(),
            title: node.title.to_owned(),
            description: node.description.to_owned(),
            latitude: node.latitude.to_owned(),
            longitude: node.longitude.to_owned(),
        })
        .collect();

    Ok(Json(NodesResponse {
        nodes: nodes_response,
    }))
}
