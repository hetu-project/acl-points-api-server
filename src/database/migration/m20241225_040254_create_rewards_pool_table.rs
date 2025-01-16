use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RewardsPool::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RewardsPool::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RewardsPool::RewardType).string().not_null())
                    .col(
                        ColumnDef::new(RewardsPool::TotalAmount)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RewardsPool::AvailableAmount)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RewardsPool::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RewardsPool::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RewardsPool::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RewardsPool {
    Table,
    Id,
    RewardType,
    TotalAmount,
    AvailableAmount,
    CreatedAt,
    UpdatedAt,
}
