use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Permission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Permission::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Permission::Name).string_len(64).not_null())
                    .col(ColumnDef::new(Permission::Resource).unique_key().string_len(64))
                    .col(ColumnDef::new(Permission::Method).string_len(64))
                    .col(
                        ColumnDef::new(Permission::ParentId)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Permission::Description).string_len(128))
                    .col(
                        ColumnDef::new(Permission::CreateAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Permission::UpdateAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Permission::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Permission {
    Table,
    Id,
    Name,
    Resource,
    Method,
    ParentId,
    Description,
    CreateAt,
    UpdateAt,
}
