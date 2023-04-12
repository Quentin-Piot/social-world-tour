mod server;

mod auth;
mod error;
mod health;
mod users;

use crate::server::start_server;
use serde::{Deserialize, Serialize};

#[tokio::main]
pub async fn main() {
    let result = start_server().await;

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
