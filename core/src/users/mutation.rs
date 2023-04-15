use ::entity::{users, users::Entity as Users};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_user(
        db: &DbConn,
        user_input: users::Model,
    ) -> Result<users::ActiveModel, DbErr> {
        users::ActiveModel {
            username: Set(user_input.username.to_owned()),
            email: Set(user_input.email.to_owned()),
            created_at: Set(user_input.created_at.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn delete_user(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let user: users::ActiveModel = Users::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;

        user.delete(db).await
    }

    pub async fn delete_all_users(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Users::delete_many().exec(db).await
    }
}
