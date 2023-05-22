use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TripsResponse {
    pub trips: Vec<TripResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TripResponse {
    pub id: i32,
    pub name: Option<String>,
    pub logo: Option<String>,
}
