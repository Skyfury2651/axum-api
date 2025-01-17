use std::sync::Arc;

use sea_orm::{Database, DbErr};

use crate::{dto::todo_dto::TodoCreateDto, entity::todos::NewTodo, entity::todos::Model as Todo, repository::todo_repository::TodoRepository};

pub struct TodoService {
    user_repo: TodoRepository,
    db_conn: Arc<Database>,
}

impl TodoService {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            user_repo: TodoRepository::new(db_conn),
            db_conn: Arc::clone(db_conn),
        }
    }

    async fn add_user(&self, payload: TodoCreateDto) -> Result<Todo, DbErr> {
        let todo = NewTodo {
            title: payload.title.clone(),
            content: payload.content.clone(),
        };

        let user = self.user_repo.register(user_model).await?;
        return Ok(user.try_into_model()?);
    }
}