pub use sea_orm_migration::prelude::*;

mod m20241204_062314_create_last_update_table;
mod m20241204_062406_create_nostr_event_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241204_062314_create_last_update_table::Migration),
            Box::new(m20241204_062406_create_nostr_event_table::Migration),
        ]
    }
}
