use sea_orm::*;

use ::entity::user_trips;

pub struct Mutation;

impl Mutation {
    pub async fn create_user_trips(
        db: &DbConn,
        trip_id: i32,
        user_id: i32,
    ) -> Result<user_trips::ActiveModel, DbErr> {
        user_trips::ActiveModel {
            user: Set(user_id.to_owned()),
            trip: Set(trip_id.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }
}
