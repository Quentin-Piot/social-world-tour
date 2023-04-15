pub use sea_orm_migration::prelude::*;

mod m20230329_121826_create_user_table;
mod m20233030_121825_create_team_table;
mod m20233030_121826_create_node_table;
mod m20233030_121827_create_user_team_map;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230329_121826_create_user_table::Migration),
            Box::new(m20233030_121825_create_team_table::Migration),
            Box::new(m20233030_121826_create_node_table::Migration),
            Box::new(m20233030_121827_create_user_team_map::Migration),
        ]
    }
}
