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
                    .table(Thread::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Thread::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Thread::Title).string().not_null())
                    .col(ColumnDef::new(Thread::Text).string().not_null())
                    .col(ColumnDef::new(Thread::Author).string().not_null())
                    .col(ColumnDef::new(Thread::ParentId).integer().null())
                    .col(
                        ColumnDef::new(Thread::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Thread::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Thread::DeletedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Thread::State)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Thread::UserId).string().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("thread_user_fk")
                            .from(Thread::Table, Thread::UserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("thread_parent_fk")
                            .from(Thread::Table, Thread::ParentId)
                            .to(Thread::Table, Thread::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("thread_parent_idx")
                    .table(Thread::Table)
                    .col(Thread::Id)
                    .col(Thread::ParentId)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        /* manager
            .rename_table(
                TableRenameStatement::new()
                    .table(Thread::Table, ThreadTmp::Table)
                    .to_owned(),
            )
            .await?; */
        manager
            .drop_table(Table::drop().table(Thread::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Thread {
    Table,
    Id,
    Title,
    Text,
    Author,
    ParentId,
    UserId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    State,
}

/* #[derive(Iden)]
pub enum ThreadTmp {
    Table,
} */
