use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::header::USER_AGENT;
use axum::http::request::Parts;

use crate::server::error::Error;

/// Extracts the JWT from the cookie token header.
pub struct UserAgentExtractor(pub Option<String>);

#[async_trait]
impl<S> FromRequestParts<S> for UserAgentExtractor
where
    S: Send + Sync,
{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        println!("checking user agent");
        if let Some(authorization_header) = parts.headers.get(USER_AGENT) {
            let header_value = authorization_header.to_str().unwrap_or("");

            Ok(UserAgentExtractor(Some(header_value.to_string())))
        } else {
            Err(Error::Unauthorized)
        }
    }
}
