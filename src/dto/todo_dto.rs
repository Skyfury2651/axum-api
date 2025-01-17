
use serde::{Deserialize, Serialize};
// A trait that the Validate derive will impl
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct TodoCreateDto {
    #[validate(required)]
    pub title: Option<String>,
    #[validate(length(max = 255, message = "Please keep todo simple"))]
    pub content: String,
}

pub struct TodoDto {
    pub id: i32,
    pub title: String,
    pub content: String,
}
