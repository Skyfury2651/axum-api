use std::sync::Arc;

use axum::async_trait;
use sea_orm::{ ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, IntoActiveModel, QueryFilter };
use crate::{ config::database::Database, entities::user::{ self, Entity as User, NewUser } };

#[async_trait]
pub trait UserRepositoryTrait {
    fn new(connection: &Arc<Database>) -> Self;
    async fn find_by_email(&self, email: String) -> Option<user::Model>;
    async fn register(&self, user: NewUser) -> Result<user::ActiveModel, DbErr>;
}

#[derive(Clone)]
pub struct UserRepository {
    pub connection: Arc<Database>,
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    fn new(connection: &Arc<Database>) -> Self {
        Self {
            connection: Arc::clone(connection),
        }
    }

    async fn find_by_email(&self, email: String) -> Option<user::Model> {
        let conn = self.connection.get_connection();
        let query = User::find().filter(user::Column::Email.eq(&email)).one(conn);
        let users = query.await.unwrap_or(None);

        return users;
    }

    async fn register(&self, user: NewUser) -> Result<user::ActiveModel, DbErr> {
        let conn = self.connection.get_connection();
        let active_model = user.into_active_model();

        active_model.save(conn).await
    }
}
