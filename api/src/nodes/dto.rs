use serde::{Deserialize, Serialize};

use social_world_tour_core::sea_orm::prelude::Decimal;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNodeInput {
    pub country_code: String,
    pub city: String,
    pub title: String,
    pub description: Option<String>,
    pub latitude: Decimal,
    pub longitude: Decimal,
    pub trip: i32,
}
