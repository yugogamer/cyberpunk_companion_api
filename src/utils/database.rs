use super::{config::Config, errors::AppErrors};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn connect_to_database(config: &Config) -> Result<Pool<Postgres>, AppErrors> {
    let db_url = config.db_url.clone();
    let pool = PgPoolOptions::new().connect(&db_url).await?;
    migrate(&pool).await?;
    Ok(pool)
}

async fn migrate(pool: &Pool<Postgres>) -> Result<(), AppErrors> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
