use crate::auth::controller::router as auth_router;
use crate::health::controller::router as health_router;
use crate::users::controller::router as user_router;

use axum::extract::{FromRef, Host};
use axum::handler::HandlerWithoutStateExt;
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Redirect};
use axum::{BoxError, Router};
use axum_server::tls_rustls::RustlsConfig;

use migration::{Migrator, MigratorTrait};
use oauth2::basic::BasicClient;

use crate::error::AppError;
use axum::routing::get;
use http::Method;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use social_world_tour_core::sea_orm::{Database, DatabaseConnection};
use std::path::PathBuf;
use std::{env, net::SocketAddr};
use tower_http::cors::{any, CorsLayer};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Copy)]
struct Ports {
    http: u16,
    https: u16,
}

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub oauth_client: BasicClient,
}

impl FromRef<AppState> for BasicClient {
    fn from_ref(state: &AppState) -> Self {
        state.oauth_client.clone()
    }
}

pub async fn start_server() -> Result<(), BoxError> {
    dotenvy::dotenv().ok();

    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tls_rustls=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");

    let ports = Ports {
        http: 2999,
        https: port.parse().unwrap(),
    };

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods(vec![Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(any());

    tokio::spawn(redirect_http_to_https(ports));

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let oauth_client = oauth_client();

    let state = AppState { conn, oauth_client };

    let app = api_router(state.clone())
        .layer(CorsLayer::permissive())
        .nest_service(
            "/static",
            ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .with_state(state);
    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("key.pem"),
    )
    .await
    .unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], ports.https));
    tracing::debug!("listening on {}", addr);
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn api_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .merge(auth_router().merge(user_router(state).merge(health_router())))
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn redirect_http_to_https(ports: Ports) {
    fn make_https(host: String, uri: Uri, ports: Ports) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&ports.http.to_string(), &ports.https.to_string());
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, ports) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], ports.http));
    tracing::debug!("http redirect listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(redirect.into_make_service())
        .await
        .unwrap();
}

fn oauth_client() -> BasicClient {
    let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID!");
    let client_secret = env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET!");
    let redirect_url = env::var("REDIRECT_URL").expect("Missing REDIRECT_URL!");
    let auth_url = env::var("AUTH_URL").expect("Missing AUTH_URL!");
    let token_url = env::var("TOKEN_URL").expect("Missing TOKEN_URL!");

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).unwrap(),
        Some(TokenUrl::new(token_url).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}
