use crate::server::start_server;

mod server;

mod auth;
mod error;
mod health;
mod nodes;
mod trips;
mod users;

#[tokio::main]
pub async fn main() {
    let result = start_server().await;

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
