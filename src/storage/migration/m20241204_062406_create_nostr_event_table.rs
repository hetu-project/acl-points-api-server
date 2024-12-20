use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NostrEvent::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(NostrEvent::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(NostrEvent::EventId).string().not_null())
                    .col(
                        ColumnDef::new(NostrEvent::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NostrEvent::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum NostrEvent {
    Table,
    Id,
    EventId,
    UpdatedAt,
}
