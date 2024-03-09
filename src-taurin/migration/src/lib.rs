pub use sea_orm_migration::prelude::*;

mod m20240221_193040_create_thread_table;
mod m20240221_203006_create_community_table;
mod m20240221_203522_create_user_table;
mod m20240221_204447_create_thread_community_table;
mod m20240223_163000_create_user_community_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240221_203522_create_user_table::Migration),
            Box::new(m20240221_203006_create_community_table::Migration),
            Box::new(m20240221_193040_create_thread_table::Migration),
            Box::new(m20240221_204447_create_thread_community_table::Migration),
            Box::new(m20240223_163000_create_user_community_table::Migration),
        ]
    }
}
