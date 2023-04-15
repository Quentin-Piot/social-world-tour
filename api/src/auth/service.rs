use crate::auth::models::{Claims, OAuthUser, KEYS};
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, TokenResponse};
use crate::error::AppError;

pub struct GenerateTokenResponse {
    pub token: String,
    pub user_data: OAuthUser
}


pub async fn generate_token_from_authorization_code(
    oauth_client: BasicClient,
    authorization_code: String,
) -> Result<GenerateTokenResponse, AppError> {
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
        sub: user_data.email.to_owned(),
        auth0_sub: user_data.sub.to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };

    let token = encode(&Header::default(), &claims, &KEYS.encoding);
    if token.is_err() {
        return Err(AppError::InvalidToken);
    }
    let token_response = GenerateTokenResponse {
        token: token.unwrap(),
        user_data
    };
    Ok(token_response)

}
