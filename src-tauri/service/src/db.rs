use migration::{Migrator, MigratorTrait};
use sea_orm::*;

pub struct DbService {
    pub db_url: String,
    pub db_file_path: String,
}

impl DbService {
    pub fn build(db_directory: String) -> Self {
        let db_file_path = format!("{}/db.sqlite", db_directory);
        let db_url = format!("sqlite://{}?mode=rwc", db_file_path);
        Self {
            db_url,
            db_file_path,
        }
    }

    pub async fn get_db_connection(&self) -> Result<DatabaseConnection, DbErr> {
        Database::connect(&self.db_url).await
    }

    pub fn does_db_exist(&self) -> bool {
        std::path::Path::new(&self.db_file_path).exists()
    }

    pub async fn init_db(&self) -> Result<(), DbErr> {
        let db = Database::connect(&self.db_url).await?;
        // Apply all migrations (seedings are also migrations)
        Migrator::up(&db, None).await?;
        Ok(())
    }
}
