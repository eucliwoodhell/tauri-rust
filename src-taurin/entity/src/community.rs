//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "community")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub name: String,
    pub image: String,
    pub bio: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
    pub state: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::thread::Entity> for Entity {
    fn to() -> RelationDef {
        super::thread_community::Relation::Thread.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::thread_community::Relation::Community.def().rev())
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_community::Relation::User.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_community::Relation::Community.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}