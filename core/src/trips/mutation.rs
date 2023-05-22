use sea_orm::{ActiveModelTrait, DbConn, DbErr, Set};

use ::entity::trips;

pub struct Mutation;

impl Mutation {
    pub async fn create_trip(db: &DbConn, trip_input: trips::Model) -> Result<trips::Model, DbErr> {
        trips::ActiveModel {
            name: Set(trip_input.name.to_owned()),
            logo: Set(trip_input.logo.to_owned()),
            created_by: Set(trip_input.created_by),
            created_at: Set(trip_input.created_at.to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await
    }
}
