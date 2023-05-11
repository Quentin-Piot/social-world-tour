use sea_orm_migration::prelude::*;

use crate::m20230329_121826_create_user_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Teams::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Teams::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Teams::Name).string())
                    .col(ColumnDef::new(Teams::Logo).string())
                    .col(ColumnDef::new(Teams::CreatedBy).integer().not_null())
                    .col(ColumnDef::new(Teams::CreatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_team_create_by_user_id")
                            .from(Teams::Table, Teams::CreatedBy)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Teams::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Teams {
    Table,
    Id,
    Name,
    Logo,
    CreatedBy,
    CreatedAt,
}
