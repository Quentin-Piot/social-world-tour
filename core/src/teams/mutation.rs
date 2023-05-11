use sea_orm::{ActiveModelTrait, DbConn, DbErr, Set};

use ::entity::teams;

pub struct Mutation;

impl Mutation {
    pub async fn create_team(db: &DbConn, team_input: teams::Model) -> Result<teams::Model, DbErr> {
        teams::ActiveModel {
            name: Set(team_input.name.to_owned()),
            logo: Set(team_input.logo.to_owned()),
            created_by: Set(team_input.created_by),
            created_at: Set(team_input.created_at.to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await
    }
}
