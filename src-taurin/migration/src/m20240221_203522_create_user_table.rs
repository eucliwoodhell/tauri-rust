use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::Bio).string().null())
                    .col(ColumnDef::new(User::Image).string().null())
                    .col(ColumnDef::new(User::Path).string().null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(User::DeletedAt).timestamp())
                    .col(
                        ColumnDef::new(User::State)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Username,
    Name,
    Bio,
    Image,
    Path,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    State,
}
