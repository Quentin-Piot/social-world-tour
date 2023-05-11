use sea_orm::*;

use ::entity::user_teams;

pub struct Mutation;

impl Mutation {
    pub async fn create_user_teams(
        db: &DbConn,
        team_id: i32,
        user_id: i32,
    ) -> Result<user_teams::ActiveModel, DbErr> {
        user_teams::ActiveModel {
            user: Set(user_id.to_owned()),
            team: Set(team_id.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }
}
