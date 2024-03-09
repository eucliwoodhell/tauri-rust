use crate::m20240221_203006_create_community_table::Community;
use crate::m20240221_203522_create_user_table::User;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserCommunity::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserCommunity::UserId).string().not_null())
                    .col(
                        ColumnDef::new(UserCommunity::CommunityId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserCommunity::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserCommunity::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(UserCommunity::DeletedAt).timestamp())
                    .col(
                        ColumnDef::new(UserCommunity::State)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .primary_key(
                        Index::create()
                            .name("user_community_pkey")
                            .col(UserCommunity::UserId)
                            .col(UserCommunity::CommunityId),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("user_community_user_fk")
                            .from(UserCommunity::Table, UserCommunity::UserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("user_community_community_fk")
                            .from(UserCommunity::Table, UserCommunity::CommunityId)
                            .to(Community::Table, Community::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserCommunity::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum UserCommunity {
    Table,
    UserId,
    CommunityId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    State,
}
