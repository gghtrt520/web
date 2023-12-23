use sea_orm::DatabaseConnection;
pub mod entity;


struct Store{
    db: DatabaseConnection,
}


impl Store{
    pub fn new(db: DatabaseConnection) -> Self{
        Self{
            db,
        }
    }
}
