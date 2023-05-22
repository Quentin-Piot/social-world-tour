use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTripInput {
    pub name: String,
    pub logo: Option<String>,
    pub users: Vec<i32>,
}
