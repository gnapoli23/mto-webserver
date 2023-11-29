use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create Request table
        manager
            .create_table(
                Table::create()
                    .table(Request::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Request::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Request::BatchId).integer())
                    .col(ColumnDef::new(Request::Value).integer().not_null())
                    .col(ColumnDef::new(Request::Status).string())
                    .to_owned(),
            )
            .await?;

        // Create User table
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .to_owned(),
            )
            .await?;
        // Add default user
        let stmt = Query::insert()
            .into_table(User::Table)
            .columns([User::Username, User::Password])
            .values_panic(["admin".into(), "admin".into()])
            .to_owned();
        manager.exec_stmt(stmt).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop Request table
        manager
            .drop_table(Table::drop().table(Request::Table).to_owned())
            .await?;

        // Drop User table
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Password,
}

#[derive(DeriveIden)]
enum Request {
    Table,
    Id,
    BatchId,
    Value,
    Status,
}
