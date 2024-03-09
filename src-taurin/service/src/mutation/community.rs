use ::entity::{
    community, community::Entity as CommunityEntity, user::Entity as UserEntity, user_community,
    user_community::Entity as UserCommunityEntity,
};
use sea_orm::{prelude::DateTimeUtc, *};

pub struct MutationCommunity;

impl MutationCommunity {
    pub async fn create_community(
        db: &DbConn,
        form_data: community::Model,
    ) -> Result<community::ActiveModel, DbErr> {
        community::ActiveModel {
            username: Set(form_data.username),
            name: Set(form_data.name),
            bio: Set(form_data.bio),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_community(
        db: &DbConn,
        form_data: community::Model,
    ) -> Result<community::Model, DbErr> {
        let community: community::ActiveModel = CommunityEntity::find_by_id(form_data.id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find community".to_string()))
            .map(Into::into)?;

        community::ActiveModel {
            id: community.id,
            username: Set(form_data.username),
            name: Set(form_data.name),
            bio: Set(form_data.bio),
            updated_at: Set(Some(DateTimeUtc::default().to_string())),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_community(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let community: community::ActiveModel = CommunityEntity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannont find community".to_string()))
            .map(Into::into)?;

        community.delete(db).await
    }

    pub async fn add_user_to_community(
        db: &DbConn,
        community_id: i32,
        user_id: String,
    ) -> Result<user_community::ActiveModel, DbErr> {
        let find_user = UserEntity::find_by_id(user_id.clone()).one(db).await?;

        let find_community = CommunityEntity::find_by_id(community_id).one(db).await?;

        if find_user.is_some() && find_community.is_some() {
            return Err(DbErr::Custom("Exist user in community".to_string()));
        }

        user_community::ActiveModel {
            user_id: Set(user_id),
            community_id: Set(community_id),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn remove_user_from_community(
        db: &DbConn,
        community_id: i32,
        user_id: String,
    ) -> Result<DeleteResult, DbErr> {
        let user_community: user_community::ActiveModel = UserCommunityEntity::find()
            .filter(user_community::Column::UserId.eq(user_id))
            .filter(user_community::Column::CommunityId.eq(community_id))
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user_community".to_string()))
            .map(Into::into)?;

        user_community.delete(db).await
    }
}
