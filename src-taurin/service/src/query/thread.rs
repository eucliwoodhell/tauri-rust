use ::entity::{
    community,
    thread::{self, Entity as ThreadEntity},
    thread_community, user,
};
use sea_orm::{sea_query::Alias, sea_query::Expr, *};

pub struct QueryThread;

impl QueryThread {
    pub async fn get_all_thread(
        db: &DbConn,
        page: u64,
        limit: u64,
    ) -> Result<Vec<thread::Model>, DbErr> {
        let thread: Vec<thread::Model> = ThreadEntity::find()
            .columns([
                thread::Column::Id,
                thread::Column::Title,
                thread::Column::Text,
                thread::Column::CreatedAt,
            ])
            .column_as(
                Expr::col((Alias::new("user_tbl"), user::Column::Id)),
                "user_id",
            )
            .column_as(
                Expr::col((Alias::new("user_tbl"), user::Column::Name)),
                "user_name",
            )
            .join_as(
                JoinType::InnerJoin,
                user::Relation::Thread.def(),
                Alias::new("user_tbl"),
            )
            .limit(limit)
            .offset((page - 1) * limit)
            .into_model()
            .all(db)
            .await?;
        Ok(thread)
    }

    pub async fn get_thread_by_id_user(db: &DbConn, user_id: String) -> Result<Vec<thread::Model>, DbErr> {
        todo!()
    }

    pub async fn get_thread_by_id(db: &DbConn, thread_id: i32) -> Result<thread::Model, DbErr> {
        todo!()
    }

    pub async fn find_by_thread_id(
        db: &DbConn,
        thread_id: i32,
    ) -> Result<Option<thread::Model>, DbErr> {
        let father_thread: thread::ActiveModel = ThreadEntity::find_by_id(thread_id)
            // .column_as(thread::Column::Id, "id")
            .columns([
                thread::Column::Id,
                thread::Column::Title,
                thread::Column::Text,
                thread::Column::CreatedAt,
            ])
            .column_as(
                Expr::col((Alias::new("user_tbl"), user::Column::Id)),
                "user_id",
            )
            .column_as(
                Expr::col((Alias::new("user_tbl"), user::Column::Name)),
                "user_name",
            )
            .join_as(
                JoinType::InnerJoin,
                user::Relation::Thread.def(),
                Alias::new("user_tbl"),
            )
            .join_as(
                JoinType::LeftJoin,
                thread_community::Relation::Thread.def(),
                Alias::new("thread_community_tbl"),
            )
            .join_as(
                JoinType::LeftJoin,
                thread_community::Entity::belongs_to(community::Entity)
                    .from(thread_community::Column::CommunityId)
                    .to(community::Column::Id)
                    .into(),
                sea_query::Alias::new("community_tbl"),
            )
            .filter(thread::Column::Id.eq(thread_id))
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Error get thread".to_string()))
            .map(Into::into)?;

        todo!()
    }

    pub async fn find_thread_by_user_id(
        db: &DbConn,
        user_id: i32,
    ) -> Result<Vec<thread::Model>, DbErr> {
        todo!()
    }

    pub async fn find_thread_by_community_id(
        db: &DbConn,
        community_id: i32,
    ) -> Result<Vec<thread::Model>, DbErr> {
        todo!()
    }
}
