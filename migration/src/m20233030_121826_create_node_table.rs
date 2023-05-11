use sea_orm_migration::prelude::*;

use crate::m20230329_121826_create_user_table::Users;
use crate::m20233030_121825_create_team_table::Teams;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Nodes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Nodes::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Nodes::CountryCode).string().not_null())
                    .col(ColumnDef::new(Nodes::City).string().not_null())
                    .col(ColumnDef::new(Nodes::Title).string().not_null())
                    .col(ColumnDef::new(Nodes::Description).string())
                    .col(ColumnDef::new(Nodes::Latitude).decimal().not_null())
                    .col(ColumnDef::new(Nodes::Longitude).decimal().not_null())
                    .col(ColumnDef::new(Nodes::Team).integer().not_null())
                    .col(ColumnDef::new(Nodes::CreatedBy).integer().not_null())
                    .col(ColumnDef::new(Nodes::CreatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_node_team_team__id")
                            .from(Nodes::Table, Nodes::Team)
                            .to(Teams::Table, Teams::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_node_create_by_user_id")
                            .from(Nodes::Table, Nodes::CreatedBy)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Nodes::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Nodes {
    Table,
    Id,
    Title,
    Description,
    CountryCode,
    City,
    Latitude,
    Longitude,
    Team,
    CreatedBy,
    CreatedAt,
}
