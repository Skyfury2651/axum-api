use crate::response::api_response::ApiErrorResponse;
use async_trait::async_trait;
use axum::Json;
use axum::extract::{ rejection::JsonRejection, FromRequest, Request };
use axum::response::{ IntoResponse, Response };
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error(transparent)] ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)] JsonRejection(#[from] JsonRejection),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedRequest<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S>
    for ValidatedRequest<T>
    where T: DeserializeOwned + Validate, S: Send + Sync
{
    type Rejection = RequestError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let value = Json::<T>::from_request(req, state).await;
        match value {
            Ok(Json(payload)) => {
                payload.validate()?;
                Ok(ValidatedRequest(payload))
            }
            Err(err) => {
                println!("Payload validate error: {}", err.body_text());
                Err(RequestError::JsonRejection(err))
            }
        }
    }
}

impl IntoResponse for RequestError {
    fn into_response(self) -> Response {
        match self {
            RequestError::ValidationError(_) => {
                ApiErrorResponse::send(400, Some(self.to_string().replace('\n', ", ")))
            }
            RequestError::JsonRejection(_) => ApiErrorResponse::send(400, Some(self.to_string())),
        }
    }
}
