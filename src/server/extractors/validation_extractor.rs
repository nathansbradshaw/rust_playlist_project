use axum::http::Request;
use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest},
    BoxError, Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::server::error::Error;

/// Validate User Request.
pub struct ValidationExtractor<T>(pub T);
#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidationExtractor<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    B: http_body::Body + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = Error;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();

        // TODO put this in a logger
        // let path = parts
        //     .extract::<MatchedPath>()
        //     .await
        //     .map(|path| path.as_str().to_owned())
        //     .ok();

        let req = Request::from_parts(parts, body);

        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => {
                // TODO phase out validator
                value.validate()?;

                Ok(Self(value.0))
            }
            // convert the error from `axum::Json` into whatever we want
            Err(rejection) => {
                // TODO put this in a logger
                // let payload = json!({
                //     "message": rejection.body_text(),
                //     "origin": "custom_extractor",
                //     "path": path,
                // });

                match rejection {
                    JsonRejection::JsonDataError(error) => {
                        Err(Error::BadRequest(error.body_text()))
                    }
                    JsonRejection::JsonSyntaxError(error) => {
                        Err(Error::BadRequest(error.body_text()))
                    }
                    JsonRejection::MissingJsonContentType(_) => Err(Error::BadRequest(
                        "Missing `Content-Type: application/json` header".to_string(),
                    )),
                    JsonRejection::BytesRejection(_) => Err(Error::BadRequest(
                        "Failed to buffer request body".to_string(),
                    )),
                    _ => Err(Error::InternalServerErrorWithContext(
                        "Unknown error parsing data".to_string(),
                    )),
                }
            }
        }
    }
}
