use super::config::Config;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("database connection error")]
    ConnectionErrors(#[from] sqlx::Error),
    #[error("database migration errors")]
    MigrateErrors(#[from] sqlx::migrate::MigrateError),
}

pub async fn connect_to_database(config: &Config) -> Result<Pool<Postgres>, DatabaseError> {
    let db_url = config.db_url.clone();
    let pool = PgPoolOptions::new().connect(&db_url).await?;
    migrate(&pool).await?;
    Ok(pool)
}

async fn migrate(pool: &Pool<Postgres>) -> Result<(), DatabaseError> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
