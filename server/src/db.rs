use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn connect_db() -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect("sqlite://db/dev.db?mode=rwc").await?;

    Ok(db)
}