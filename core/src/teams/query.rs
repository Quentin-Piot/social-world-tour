use sea_orm::{DbConn, DbErr, EntityTrait, JoinType, QuerySelect};

use ::entity::prelude::Teams;
use ::entity::teams;
use ::entity::teams::Model;
use ::entity::user_teams;

pub struct Query;

impl Query {
    pub async fn find_teams_by_user_id(db: &DbConn, id: i32) -> Result<Vec<Model>, DbErr> {
        Teams::find()
            .join_rev(
                JoinType::InnerJoin,
                user_teams::Entity::belongs_to(teams::Entity)
                    .from(user_teams::Column::Team)
                    .to(teams::Column::Id)
                    .into(),
            )
            .all(db)
            .await
    }
}
