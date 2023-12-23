use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                    .col(ColumnDef::new(User::Username).string_len(64).unique_key().not_null())
                    .col(ColumnDef::new(User::Password).string_len(128).not_null())
                    .col(ColumnDef::new(User::Nickname).string_len(64).not_null())
                    .col(ColumnDef::new(User::Email).string_len(64).not_null())
                    .col(ColumnDef::new(User::Phone).string_len(32).unique_key().not_null())
                    .col(ColumnDef::new(User::Avatar).string_len(128))
                    .col(ColumnDef::new(User::Status).tiny_unsigned().default(0))
                    .col(
                        ColumnDef::new(User::CreateAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(User::UpdateAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(User::DeleteAt).date_time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
    Nickname,
    Email,
    Phone,
    Avatar,
    Status,
    CreateAt,
    UpdateAt,
    DeleteAt,
}
