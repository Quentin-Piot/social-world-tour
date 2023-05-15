use sea_orm::{ActiveModelTrait, DbConn, DbErr, Set};

use ::entity::nodes;

pub struct Mutation;

impl Mutation {
    pub async fn create_node(db: &DbConn, node_input: nodes::Model) -> Result<nodes::Model, DbErr> {
        nodes::ActiveModel {
            country_code: Set(node_input.country_code.to_owned()),
            city: Set(node_input.city.to_owned()),
            title: Set(node_input.title.to_owned()),
            description: Set(node_input.description.to_owned()),
            latitude: Set(node_input.latitude.to_owned()),
            longitude: Set(node_input.longitude.to_owned()),
            team: Set(node_input.team.to_owned()),
            created_by: Set(node_input.created_by),
            created_at: Set(node_input.created_at.to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await
    }
}
