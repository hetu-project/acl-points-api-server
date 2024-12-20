//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "activity_tasks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub activity_id: i32,
    pub project_id: i32,
    pub event_type: String,
    pub contract_address: Option<String>,
    pub chain: Option<String>,
    pub online_time: Option<i32>,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub is_scheduled: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
