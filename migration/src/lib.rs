use sea_orm_migration::{MigrationTrait, MigratorTrait};

mod m20240819_000001_sys_user;

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240819_000001_sys_user::Migration)]
    }
}
