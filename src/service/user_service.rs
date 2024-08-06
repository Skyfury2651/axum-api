use sea_orm::{ DbErr, RuntimeErr, TryIntoModel };

use crate::config::database::Database;
use crate::dto::user_dto::{ UserReadDto, UserRegisterDto };
use crate::entities::user::{ Model as User, NewUser };
use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::error::user_error::UserError;
use crate::repository::user_repository::{ UserRepository, UserRepositoryTrait };
use std::sync::Arc;
#[allow(dead_code)]
#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
    db_conn: Arc<Database>,
}

impl UserService {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            user_repo: UserRepository::new(db_conn),
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn create_user(&self, payload: UserRegisterDto) -> Result<UserReadDto, ApiError> {
        let response = self.user_repo.find_by_email(payload.email.to_owned()).await;
        return match response {
            Some(_) => Err(UserError::UserAlreadyExists)?,
            None => {
                let hashed_user = match UserService::generate_user_from_payload(payload) {
                    Ok(user) => user,
                    Err(error) => Err(DbError::SomethingWentWrong(error.to_string()))?,
                };

                let user = self.add_user(hashed_user).await;

                return match user {
                    Ok(user) => Ok(UserReadDto::from(user)),
                    Err(error) => {
                        match error {
                            DbErr::Exec(RuntimeErr::SqlxError(error)) =>
                                match error {
                                    sqlx::Error::Database(e) => {
                                        Err(DbError::UniqueConstraintViolation(e.to_string()))?
                                    }
                                    _ => panic!("Unexpected sqlx::Error kind"),
                                }
                            _ => Err(DbError::SomethingWentWrong(error.to_string()))?,
                        }
                    }
                };
            }
        };
    }

    async fn add_user(&self, payload: UserRegisterDto) -> Result<User, DbErr> {
        let user_model = NewUser {
            last_name: payload.last_name.clone(),
            first_name: payload.first_name.clone(),
            email: payload.email,
            password: payload.password,
            user_name: payload.user_name,
        };

        let user = self.user_repo.register(user_model).await?;
        return Ok(user.try_into_model()?);
    }

    pub fn verify_password(&self, user: &User, password: &str) -> bool {
        return bcrypt::verify(password, &user.password).unwrap_or(false);
    }

    pub fn generate_user_from_payload(
        payload: UserRegisterDto
    ) -> Result<UserRegisterDto, DbError> {
        let mut user_dto = payload.clone();

        match bcrypt::hash(payload.password, 12) {
            Ok(password) => {
                user_dto.password = password;
            }
            Err(e) => {
                return Err(DbError::SomethingWentWrong(e.to_string()));
            }
        }

        Ok(user_dto)
    }
}
