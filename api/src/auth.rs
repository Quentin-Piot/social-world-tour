use axum::{
    extract::{Extension, Json},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    AddExtensionLayer, Router,
};
use oauth2::{
    basic::BasicClient, reqwest::http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret,
    CsrfToken, RedirectUrl, TokenUrl,
};
use reqwest::{header::AUTHORIZATION, Client, Response, StatusCode as HttpStatus};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserInfo {
    sub: String, // user id
    name: String,
    email: String,
}

#[derive(Debug)]
enum AuthError {
    MissingAuthorizationHeader,
    InvalidAuthorizationHeader,
    FailedToGetUserInfo,
    InvalidUserInfo,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Self::MissingAuthorizationHeader => StatusCode::BAD_REQUEST,
            Self::InvalidAuthorizationHeader => StatusCode::UNAUTHORIZED,
            Self::FailedToGetUserInfo => StatusCode::UNAUTHORIZED,
            Self::InvalidUserInfo => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}

async fn authenticate(
    auth_header: Option<String>,
    auth0_client: Extension<Auth0Client>,
) -> Result<UserInfo, AuthError> {
    let token = match auth_header {
        Some(header) => {
            let parts: Vec<&str> = header.splitn(2, ' ').collect();
            if parts.len() != 2 || parts[0] != "Bearer" {
                return Err(AuthError::InvalidAuthorizationHeader);
            }
            parts[1].to_owned()
        }
        None => return Err(AuthError::MissingAuthorizationHeader),
    };

    let client = &auth0_client.0.client;
    let token_info = client
        .verify_access_token(&token, &["openid email profile".to_string()])
        .await
        .map_err(|_| AuthError::InvalidAuthorizationHeader)?;

    let user_info = client
        .get_user_info(&token_info.access_token)
        .await
        .map_err(|_| AuthError::FailedToGetUserInfo)?;

    let user_info: UserInfo =
        serde_json::from_str(&user_info).map_err(|_| AuthError::InvalidUserInfo)?;

    Ok(user_info)
}

async fn handle_auth_error(err: HandlerError<AuthError>) -> axum::http::Response<String> {
    let (error, status_code) = match err {
        HandlerError::Response(resp) => {
            let status_code = resp.status();
            let error = match resp.into_body().into_string().await {
                Ok(error) => error,
                Err(_) => AuthError::InvalidAuthorizationHeader.to_string(),
            };
            (error, status_code)
        }
        HandlerError::Inner(error) => (error.to_string(), StatusCode::INTERNAL_SERVER_ERROR),
    };
    axum::http::Response::builder()
        .status(status_code)
        .body(error)
        .unwrap()
}

pub struct Auth0Client {
    client: BasicClient,
    userinfo_url: String,
}

impl Auth0Client {
    pub fn new(
        domain: &str,
        client_id: &str,
        client_secret: &str,
        redirect_url: &str,
        userinfo_url: &str,
    ) -> Auth0Client {
        let client_id = ClientId::new(client_id.to_owned());
        let client_secret = ClientSecret::new(client_secret.to_owned());
        let auth_url = AuthUrl::new(format!("https://{}/authorize", domain)).unwrap();
        let token_url = TokenUrl::new(format!("https://{}/oauth/token", domain)).unwrap();
        let redirect_uri = RedirectUrl::new(redirect_url.to_owned()).unwrap();

        let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
            .set_redirect_uri(redirect_uri);

        Auth0Client {
            client,
            userinfo_url: userinfo_url.to_owned(),
        }
    }
}

async fn login_handler(auth0_client: Extension<Auth0Client>) -> Result<Json<UserInfo>, AuthError> {
    let authorize_url = auth0_client
        .0
        .client
        .authorize_url(CsrfToken::new_random)
        .url();

    Err(AuthError::MissingAuthorizationHeader)
        .map_err(|err| HandlerError::Response(err.into_response()))?;

    let client = Client::builder().build().unwrap();

    let resp = client.get(authorize_url.as_str()).send().await.unwrap();

    let resp_url = resp.url().to_owned();
    let resp_headers = resp.headers().clone();
    let resp_status = resp.status();
    if resp_status != HttpStatus::FOUND {
        return Err(AuthError::InvalidAuthorizationHeader);
    }

    let location_header = resp_headers.get(header::LOCATION).unwrap();
    let code = location_header
        .to_str()
        .unwrap()
        .rsplit('=')
        .next()
        .unwrap()
        .to_owned();

    let token_result = auth0_client
        .0
        .client
        .exchange_code(AuthorizationCode::new(code))
        .request(http_client)
        .await
        .map_err(|_| AuthError::FailedToGetUserInfo)?;

    let user_info = auth0_client
        .0
        .client
        .get_user_info(&token_result.access_token)
        .await
        .map_err(|_| AuthError::FailedToGetUserInfo)?;

    let user_info: UserInfo =
        serde_json::from_str(&user_info).map_err(|_| AuthError::InvalidUserInfo)?;

    Ok(Json(user_info))
}

pub fn auth_routes() -> Router<AddExtensionLayer<Auth0Client>> {
    Router::new()
        .route("/login", get(login_handler))
        .layer(AddExtensionLayer::new(Auth0Client::new(
            "YOUR_AUTH0_DOMAIN",
            "YOUR_CLIENT_ID",
            "YOUR_CLIENT_SECRET",
            "http://localhost:3000/auth/callback",
            "https://YOUR_AUTH0_DOMAIN/userinfo",
        )))
        .layer(axum::AddExtensionLayer::new(
            move |request: axum::extract::RequestParts| {
                let auth0_client = request
                    .extensions()
                    .get::<Extension<Auth0Client>>()
                    .unwrap();
                auth0_client.clone()
            },
        ))
}
