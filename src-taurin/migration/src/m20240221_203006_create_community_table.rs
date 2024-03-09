use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Community::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Community::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Community::Username)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Community::Name).string().not_null())
                    .col(ColumnDef::new(Community::Image).string().not_null())
                    .col(ColumnDef::new(Community::Bio).string().not_null())
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
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Community::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Community {
    Table,
    Id,
    Username,
    Name,
    Image,
    Bio,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    State,
}
