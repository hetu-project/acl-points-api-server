//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub uid: String,
    pub name: String,
    #[sea_orm(unique)]
    pub email: String,
    #[sea_orm(unique)]
    pub address: Option<String>,
    pub password: Option<String>,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    pub provider: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
