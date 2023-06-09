//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::nodes::Entity")]
    Nodes,
    #[sea_orm(has_many = "super::trips::Entity")]
    Trips,
    #[sea_orm(has_many = "super::user_trips::Entity")]
    UserTrips,
}

impl Related<super::nodes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Nodes.def()
    }
}

impl Related<super::trips::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Trips.def()
    }
}

impl Related<super::user_trips::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserTrips.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
