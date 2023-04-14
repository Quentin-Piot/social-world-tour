use crate::auth::models::{Claims, OAuthUser, KEYS};
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, TokenResponse};

pub async fn generate_token_from_authorization_code(
    oauth_client: BasicClient,
    authorization_code: String,
) -> jsonwebtoken::errors::Result<String> {
    let token_result = oauth_client
        .exchange_code(AuthorizationCode::new(authorization_code))
        .request_async(async_http_client)
        .await
        .unwrap();
    let access_token = token_result.access_token().secret();

    let client = reqwest::Client::new();
    let user_data = client
        .get("https://dev-social-media-tour.eu.auth0.com/userinfo")
        .bearer_auth(access_token)
        .send()
        .await
        .unwrap()
        .json::<OAuthUser>()
        .await
        .unwrap();

    let claims = Claims {
        sub: user_data.email,
        given_name: user_data.given_name,
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };

    encode(&Header::default(), &claims, &KEYS.encoding)
}
