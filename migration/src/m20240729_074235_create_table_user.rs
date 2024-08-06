use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(ColumnDef::new(User::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(User::FirstName).string())
                .col(ColumnDef::new(User::LastName).string())
                .col(ColumnDef::new(User::UserName).string().not_null().unique_key())
                .col(ColumnDef::new(User::Email).string().unique_key())
                .col(ColumnDef::new(User::Password).string())
                .col(ColumnDef::new(User::CreatedAt).timestamp().default(Expr::current_timestamp()))
                .col(ColumnDef::new(User::UpdatedAt).timestamp().default(Expr::current_timestamp()))
                .col(ColumnDef::new(User::IsActive).tiny_integer().not_null().default(0))
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    FirstName,
    LastName,
    UserName,
    Email,
    Password,
    CreatedAt,
    UpdatedAt,
    IsActive,
}
