use axum::Json;

use crate::{dto::todo_dto::{TodoCreateDto, TodoDto}, error::{api_error::ApiError, request_error::ValidatedRequest}};


pub async fn create(ValidatedRequest(payload): ValidatedRequest<TodoCreateDto>) -> Result<axum::Json<bool>, ApiError>{
    // let user = state.user_service.create_user(payload).await?;
    

    Ok(Json(true))
}