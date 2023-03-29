use ::entity::{users, users::Entity as Users};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_user(
        db: &DbConn,
        form_data: users::Model,
    ) -> Result<users::ActiveModel, DbErr> {
        users::ActiveModel {
            username: Set(form_data.username.to_owned()),
            email: Set(form_data.email.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_user_by_id(
        db: &DbConn,
        id: i32,
        form_data: users::Model,
    ) -> Result<users::Model, DbErr> {
        let user: users::ActiveModel = Users::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;

        users::ActiveModel {
            id: user.id,
            username: Set(form_data.username.to_owned()),
            email: Set(form_data.email.to_owned()),
        }
        .update(db)
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
