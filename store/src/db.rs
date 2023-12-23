use error::Result;
use sea_orm::{Database, DatabaseConnection};
use error::Error;

pub async fn new() -> Result<DatabaseConnection> {
    let connect = std::env::var("DATABASE_URL").map_err(|_| Error::NoEnvirmentError)?;
    Database::connect(connect)
        .await
        .map_err(|_| error::Error::DatabaseConnectionError)
}
