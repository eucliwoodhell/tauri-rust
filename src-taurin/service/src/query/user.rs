use ::entity::{user, user::Entity as UserEntity};
use sea_orm::*;

pub struct QueryUser;

impl QueryUser {
    pub async fn find_user_by_id(db: &DbConn, id: String) -> Result<Option<user::Model>, DbErr> {
        UserEntity::find_by_id(id).one(db).await
    }
}
