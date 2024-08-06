use crate::error::{ api_error::ApiError, token_error::TokenError, user_error::UserError };
use crate::repository::user_repository::UserRepositoryTrait;
use crate::service::token_service::TokenServiceTrait;
use crate::state::token_state::TokenState;
use axum::extract::State;
use jsonwebtoken::errors::ErrorKind;
use axum::{ response::IntoResponse, middleware::Next, extract::Request };

pub async fn auth(
    State(state): State<TokenState>,
    mut req: Request,
    next: Next
) -> Result<impl IntoResponse, ApiError> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(TokenError::MissingToken)?;
    };
    let mut header = auth_header.split_whitespace();
    let token = header.next();

    let token = match token {
        Some(token) => token,
        None => {
            return Err(TokenError::MissingToken)?;
        }
    };

    match state.token_service.retrieve_token_claims(token) {
        Ok(token_data) => {
            let user = state.user_repo.find_by_email(token_data.claims.email).await;
            match user {
                Some(user) => {
                    req.extensions_mut().insert(user);
                    Ok(next.run(req).await)
                }
                None => {
                    return Err(UserError::UserNotFound)?;
                }
            }
        }
        Err(err) => {
            return match err.kind() {
                ErrorKind::ExpiredSignature => Err(TokenError::TokenExpired)?,
                _ => Err(TokenError::InvalidToken(token.parse().unwrap_or_default()))?,
            };
        }
    }
}
