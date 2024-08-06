use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager.create_table(
            Table::create()
                .table(todos::Table::Todos)
                .if_not_exists()
                .col(
                    ColumnDef::new(todos::Column::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                )
                .col(ColumnDef::new(todos::Column::Title).string().not_null())
                .col(ColumnDef::new(todos::Column::Content).string().not_null())
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(todos::Table::Todos).to_owned()).await
    }
}

mod todos {
    use sea_orm_migration::prelude::*;

    #[derive(DeriveIden)]
    pub enum Table {
        Todos,
    }

    #[derive(DeriveIden)]
    pub enum Column {
        Id,
        Title,
        Content,
    }
}
