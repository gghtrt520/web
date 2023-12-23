use error::Result;
use sea_orm::DatabaseConnection;

pub mod db;
pub mod entity;


#[derive(Debug,Clone)]
pub struct Store {
    db: DatabaseConnection,
}

impl Store {
    pub async fn new() -> Result<Self> {
        let db = db::new().await?;
        Ok(Self { db })
    }
}
