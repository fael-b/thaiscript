pub use sea_orm_migration::prelude::*;

mod m20240316_104818_create_letter_variant_table;
mod m20240316_131723_seed_letter_variant_data;
mod m20240317_142206_create_review_outcome_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240316_104818_create_letter_variant_table::Migration),
            Box::new(m20240316_131723_seed_letter_variant_data::Migration),
            Box::new(m20240317_142206_create_review_outcome_table::Migration),
        ]
    }
}
