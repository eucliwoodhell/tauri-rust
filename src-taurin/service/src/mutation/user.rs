use ::entity::{user, user::Entity as UserEntity};
use sea_orm::*;

pub struct MutationUser;

impl MutationUser {
    pub async fn create_user(
        db: &DbConn,
        form_data: user::Model,
    ) -> Result<user::ActiveModel, DbErr> {
        user::ActiveModel {
            id: Set(form_data.id),
            name: Set(form_data.name),
            bio: Set(form_data.bio),
            path: Set(form_data.path),
            image: Set(form_data.image),
            username: Set(form_data.username),
        }
        .save(db)
        .await
    }

    pub async fn update_user(db: &DbConn, form_data: user::Model) -> Result<user::Model, DbErr> {
        let user: user::ActiveModel = UserEntity::find_by_id(form_data.id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user".to_string()))
            .map(Into::into)?;

        user::ActiveModel {
            id: user.id,
            name: Set(form_data.name),
            bio: Set(form_data.bio),
            path: Set(form_data.path),
            image: Set(form_data.image),
            username: Set(form_data.username),
        }
        .update(db)
        .await
    }

    pub async fn delete_user(db: &DbConn, id: String) -> Result<DeleteResult, DbErr> {
        let user: user::ActiveModel = UserEntity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user".to_string()))
            .map(Into::into)?;

        user.delete(db).await
    }
}
