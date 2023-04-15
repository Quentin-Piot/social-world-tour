use ::entity::users;
use ::entity::prelude::Users;
use ::entity::users::Model;
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_user_by_id(db: &DbConn, id: i32) -> Result<Option<Model>, DbErr> {
        Users::find_by_id(id).one(db).await
    }
    pub async fn find_user_by_email(db: &DbConn, email: &str) -> Result<Option<Model>, DbErr> {
        Users::find().filter(users::Column::Email.contains(email)).one(db).await
    }
}
