use ::entity::{users, users::Entity as Users};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_user_by_id(db: &DbConn, id: i32) -> Result<Option<users::Model>, DbErr> {
        Users::find_by_id(id).one(db).await
    }
}
