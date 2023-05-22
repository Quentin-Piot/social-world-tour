use sea_orm::{entity::ColumnTrait, entity::EntityTrait, query::QueryFilter, DbConn, DbErr};

use ::entity::nodes;
use ::entity::nodes::Model;
use ::entity::prelude::Nodes;

pub struct Query;

impl Query {
    pub async fn find_nodes_by_trip_id(db: &DbConn, trip_id: i32) -> Result<Vec<Model>, DbErr> {
        Nodes::find()
            .filter(nodes::Column::Trip.eq(trip_id))
            .all(db)
            .await
    }
}
