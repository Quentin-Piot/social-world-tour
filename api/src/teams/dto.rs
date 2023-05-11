use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamInput {
    pub name: String,
    pub logo: Option<String>,
    pub users: Vec<i32>,
}
