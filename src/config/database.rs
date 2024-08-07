use crate::parameter;

use sea_orm::{ DatabaseConnection, ConnectionTrait, DbBackend, DbErr, Statement };

pub struct Database {
    pub connection: DatabaseConnection,
}

impl Database {
    pub async fn init() -> Result<Database, DbErr> {
        let database_url: &str = parameter::get("DATABASE_URL").leak();
        let db_name: &str = parameter::get("DATABASE_NAME").leak();
        let db: Result<DatabaseConnection, DbErr> = sea_orm::Database::connect(database_url).await;

        let db: DatabaseConnection = match db {
            Ok(connection) => {
                println!("Successfully connected to the database.");
                connection
            }
            Err(error) => {
                println!("{:#?}", error);
                panic!("Problem opening the file: {error:?}");
            }
        };

        let db = &(match db.get_database_backend() {
            DbBackend::MySql => {
                db.execute(
                    Statement::from_string(
                        db.get_database_backend(),
                        format!("CREATE DATABASE IF NOT EXISTS `{}`;", db_name)
                    )
                ).await?;

                let url = format!("{}/{}", database_url, db_name);
                sea_orm::Database::connect(&url).await?
            }
            DbBackend::Postgres => {
                db.execute(
                    Statement::from_string(
                        db.get_database_backend(),
                        format!("DROP DATABASE IF EXISTS \"{}\";", db_name)
                    )
                ).await?;
                db.execute(
                    Statement::from_string(
                        db.get_database_backend(),
                        format!("CREATE DATABASE \"{}\";", db_name)
                    )
                ).await?;

                let url = format!("{}/{}", database_url, db_name);
                sea_orm::Database::connect(&url).await?
            }
            DbBackend::Sqlite => db,
        });

        Ok(Database { connection: db.clone() })
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}
