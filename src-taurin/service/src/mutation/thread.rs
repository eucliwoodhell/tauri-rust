use ::entity::{
    community::Entity as CommunityEntity, thread, thread::Entity as ThreadEntity,
    thread_community, thread_community::Entity as ThreadCommunityEntity,
};
use sea_orm::*;

pub struct MutationThread;

impl MutationThread {
    pub async fn create_thread(
        db: &DbConn,
        form_data: thread::Model,
    ) -> Result<thread::ActiveModel, DbErr> {
        thread::ActiveModel {
            id: Set(form_data.id),
            title: Set(form_data.title),
            text: Set(form_data.text),
            author: Set(form_data.author),
            parent_id: Set(form_data.parent_id),
            user_id: Set(form_data.user_id),
            state: Set(true),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_thread(
        db: &DbConn,
        form_data: thread::Model,
    ) -> Result<thread::Model, DbErr> {
        let thread: thread::ActiveModel = ThreadEntity::find_by_id(form_data.id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user".to_string()))
            .map(Into::into)?;

        thread::ActiveModel {
            id: thread.id,
            title: Set(form_data.title),
            text: Set(form_data.text),
            author: Set(form_data.author),
            parent_id: Set(form_data.parent_id),
            user_id: Set(form_data.user_id),
            state: Set(true),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_thread(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let user: thread::ActiveModel = ThreadEntity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannont find user".to_string()))
            .map(Into::into)?;

        user.delete(db).await
    }

    pub async fn add_thread_to_community(
        db: &DbConn,
        thread_id: i32,
        community_id: i32,
    ) -> Result<thread_community::ActiveModel, DbErr> {
        let find_thread = ThreadEntity::find_by_id(thread_id).one(db).await?;

        let find_community = CommunityEntity::find_by_id(community_id).one(db).await?;

        if find_community.is_some() && !find_thread.is_some() {
            return Err(DbErr::Custom("Exist thread in this community".to_string()));
        }

        thread_community::ActiveModel {
            thread_id: Set(thread_id),
            community_id: Set(community_id),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn remove_thread_from_community(
        db: &DbConn,
        thread_id: i32,
        community_id: i32,
    ) -> Result<DeleteResult, DbErr> {
        ThreadCommunityEntity::delete_many()
            .filter(
                thread_community::Column::ThreadId
                    .eq(thread_id)
                    .and(thread_community::Column::CommunityId.eq(community_id)),
            )
            .exec(db)
            .await
    }
}
