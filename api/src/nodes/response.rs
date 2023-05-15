use serde::{Deserialize, Serialize};

use social_world_tour_core::sea_orm::prelude::Decimal;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodesResponse {
    pub nodes: Vec<NodeResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeResponse {
    pub id: i32,
    pub country_code: String,
    pub city: String,
    pub title: String,
    pub description: Option<String>,
    pub latitude: Decimal,
    pub longitude: Decimal,
}
