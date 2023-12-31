use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RolePermission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RolePermission::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RolePermission::RoleId).integer().not_null())
                    .col(
                        ColumnDef::new(RolePermission::PermissionId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RolePermission::CreateAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(RolePermission::UpdateAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("role-permission-id")
                            .col(RolePermission::RoleId)
                            .col(RolePermission::PermissionId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RolePermission::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RolePermission {
    Table,
    Id,
    RoleId,
    PermissionId,
    CreateAt,
    UpdateAt,
}
