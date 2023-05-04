use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuthorizeResponse {
    pub(crate) callback_url: String,
}
