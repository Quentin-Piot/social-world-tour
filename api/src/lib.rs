mod flash;
mod routes;

use axum::{Router, Server};
use routes::{posts as posts_routes, users as users_routes};

use migration::{Migrator, MigratorTrait};
use serde::{Deserialize, Serialize};
use social_world_tour_core::sea_orm::{Database, DatabaseConnection};
use std::str::FromStr;
use std::{env, net::SocketAddr};
use tera::Tera;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

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
async fn start() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        .expect("Tera initialization failed");

    let state = AppState { templates, conn };

    let app = api_router()
        .nest_service(
            "/static",
            ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .layer(CookieManagerLayer::new())
        .with_state(state);

    let addr = SocketAddr::from_str(&server_url).unwrap();
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}

fn api_router() -> Router<AppState> {
    users_routes::router().merge(posts_routes::router())
}
