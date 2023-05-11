use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamsResponse {
    pub teams: Vec<TeamResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamResponse {
    pub name: Option<String>,
    pub logo: Option<String>,
}
