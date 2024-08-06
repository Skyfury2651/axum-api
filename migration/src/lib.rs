pub use sea_orm_migration::prelude::*;

mod m20240727_172256_create_table_todo;
mod m20240729_074235_create_table_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240727_172256_create_table_todo::Migration),
            Box::new(m20240729_074235_create_table_user::Migration),
        ]
    }
}
