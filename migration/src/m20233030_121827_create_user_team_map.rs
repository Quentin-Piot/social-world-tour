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
                    .table(UserTeams::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserTeams::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserTeams::User).integer().not_null())
                    .col(ColumnDef::new(UserTeams::Team).integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_map_user_team_team_id")
                            .from(UserTeams::Table, UserTeams::Team)
                            .to(Teams::Table, Teams::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_map_user_team_user_id")
                            .from(UserTeams::Table, UserTeams::User)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserTeams::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum UserTeams {
    Table,
    Id,
    User,
    Team,
}
