use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Candy::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Candy::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Candy::UserUid).string().not_null())
                    .col(ColumnDef::new(Candy::Amount).integer().not_null())
                    .col(
                        ColumnDef::new(Candy::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Candy::Description).string().null())
                    .col(
                        ColumnDef::new(Candy::ExpiresAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Candy::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Candy {
    Table,
    Id,
    UserUid,
    Amount,
    Description,
    ExpiresAt,
    CreatedAt,
}
