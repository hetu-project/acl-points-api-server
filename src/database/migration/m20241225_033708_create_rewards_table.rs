use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Rewards::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Rewards::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Rewards::UserUid).integer().not_null())
                    .col(ColumnDef::new(Rewards::RewardType).string().not_null())
                    .col(ColumnDef::new(Rewards::Amount).integer().not_null())
                    .col(
                        ColumnDef::new(Rewards::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Rewards::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Rewards {
    Table,
    Id,
    UserUid,
    RewardType,
    Amount,
    CreatedAt,
}
