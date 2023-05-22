use sea_orm_migration::prelude::*;

use crate::m20230329_121826_create_user_table::Users;
use crate::m20233030_121825_create_trip_table::Trips;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserTrips::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserTrips::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserTrips::User).integer().not_null())
                    .col(ColumnDef::new(UserTrips::Trip).integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_map_user_trip_trip_id")
                            .from(UserTrips::Table, UserTrips::Trip)
                            .to(Trips::Table, Trips::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_map_user_trip_user_id")
                            .from(UserTrips::Table, UserTrips::User)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserTrips::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum UserTrips {
    Table,
    Id,
    User,
    Trip,
}
