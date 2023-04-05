mod server;

mod users;
mod error;

use crate::server::start_server;
use serde::{Deserialize, Serialize};
use social_world_tour_core::sea_orm::DatabaseConnection;
use tera::Tera;

#[derive(Clone)]
pub struct AppState {
    templates: Tera,
    conn: DatabaseConnection,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FlashData {
    kind: String,
    message: String,
}

#[tokio::main]
pub async fn main() {
    let result = start_server().await;

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
