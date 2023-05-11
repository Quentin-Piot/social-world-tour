use sea_orm::{entity::ColumnTrait, entity::EntityTrait, query::QueryFilter, DbConn, DbErr};

use ::entity::nodes;
use ::entity::nodes::Model;
use ::entity::prelude::Nodes;

pub struct Query;

impl Query {
    pub async fn find_nodes_by_team_id(db: &DbConn, team_id: i32) -> Result<Vec<Model>, DbErr> {
        Nodes::find()
            .filter(nodes::Column::Team.eq(team_id))
            .all(db)
            .await
    }
}
