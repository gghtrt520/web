pub use sea_orm_migration::prelude::*;

mod m20231223_012325_user;
mod m20231223_012454_permission;
mod m20231223_012510_role;
mod m20231223_012517_user_role;
mod m20231223_012533_role_permission;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231223_012325_user::Migration),
            Box::new(m20231223_012454_permission::Migration),
            Box::new(m20231223_012510_role::Migration),
            Box::new(m20231223_012517_user_role::Migration),
            Box::new(m20231223_012533_role_permission::Migration),
        ]
    }
}
