pub use sea_orm_migration::prelude::*;

mod m20231129_132637_create_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20231129_132637_create_tables::Migration)]
    }
}
