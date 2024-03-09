use crate::m20240221_193040_create_thread_table::Thread;
use crate::m20240221_203006_create_community_table::Community;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(ThreadCommunity::Table)
                    .col(
                        ColumnDef::new(ThreadCommunity::ThreadId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ThreadCommunity::CommunityId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Community::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Community::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Community::DeletedAt).timestamp())
                    .col(
                        ColumnDef::new(Community::State)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .primary_key(
                        Index::create()
                            .name("thread_community_pkey")
                            .col(ThreadCommunity::ThreadId)
                            .col(ThreadCommunity::CommunityId),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("thread_community_thread_fk")
                            .from(ThreadCommunity::Table, ThreadCommunity::ThreadId)
                            .to(Thread::Table, Thread::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("thread_community_community_fk")
                            .from(ThreadCommunity::Table, ThreadCommunity::CommunityId)
                            .to(Community::Table, Community::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ThreadCommunity::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum ThreadCommunity {
    Table,
    ThreadId,
    CommunityId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    State,
}
