use sea_orm::sea_query::{Expr, IntoCondition};
use sea_orm::{DbConn, DbErr, EntityTrait, JoinType, QuerySelect};

use ::entity::prelude::Trips;
use ::entity::trips;
use ::entity::trips::Model;
use ::entity::user_trips;

pub struct Query;

impl Query {
    pub async fn find_trips_by_user_id(db: &DbConn, id: i32) -> Result<Vec<Model>, DbErr> {
        Trips::find()
            .join_rev(
                JoinType::InnerJoin,
                user_trips::Entity::belongs_to(trips::Entity)
                    .from(user_trips::Column::Trip)
                    .to(trips::Column::Id)
                    .on_condition(move |_left, _right| {
                        Expr::col(user_trips::Column::User).eq(id).into_condition()
                    })
                    .into(),
            )
            .all(db)
            .await
    }
}
