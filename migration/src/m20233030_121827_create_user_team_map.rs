use crate::m20230329_121826_create_user_table::Users;
use crate::m20233030_121825_create_team_table::Team;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserTeam::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserTeam::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserTeam::User).integer().not_null())
                    .col(ColumnDef::new(UserTeam::Team).integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_map_user_team_team_id")
                            .from(UserTeam::Table, UserTeam::Team)
                            .to(Team::Table, Team::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_map_user_team_user_id")
                            .from(UserTeam::Table, UserTeam::User)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserTeam::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum UserTeam {
    Table,
    Id,
    User,
    Team,
}
