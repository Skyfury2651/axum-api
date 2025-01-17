use std::sync::Arc;

use axum::async_trait;
use sea_orm::Database;
#[derive(Clone)]
pub struct TodoRepository {
    pub connection: Arc<Database>,
}

#[async_trait]
pub trait TodoRepositoryTrait {
    fn new(connection: &Arc<Database>) -> Self;
    async fn create();
}

#[async_trait]
impl TodoRepositoryTrait for TodoRepository {
    fn new(connection: &Arc<Database>) -> Self {
        Self {
            connection: Arc::clone(connection),
        }
    }

    async fn create() {
        
    }
}
