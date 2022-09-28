use sqlx::PgPool;

use crate::{entities::stats::*, utils::errors::AppErrors};

pub async fn create_stats_template(
    conn: &PgPool,
    stats: CreateStatsTemplate,
) -> Result<(), AppErrors> {
    sqlx::query!(
        "INSERT INTO stats_template (name, description, type) VALUES ($1, $2, $3)",
        stats.name,
        stats.description,
        stats.types,
    )
    .execute(conn)
    .await?;
    Ok(())
}

pub async fn get_one_stats_template(
    conn: &PgPool,
    id: i32,
) -> Result<Option<StatsTemplate>, AppErrors> {
    let result = sqlx::query_as!(
        StatsTemplate,
        "SELECT st.id, st.name, st.description, t.name as types
            FROM stats_template AS st
            inner join \"types\" as t on t.id = st.type
            where st.id = $1",
        id,
    )
    .fetch_optional(conn)
    .await?;
    Ok(result)
}

pub async fn get_all_stats_template(
    conn: &PgPool,
    limite: i64,
    offset: i64,
) -> Result<Vec<StatsTemplate>, AppErrors> {
    let result = sqlx::query_as!(
        StatsTemplate,
        "SELECT st.id, st.name, st.description, t.name as types
            FROM stats_template AS st
            inner join \"types\" as t on t.id = st.type
            LIMIT $1 OFFSET $2",
        limite,
        offset,
    )
    .fetch_all(conn)
    .await?;
    Ok(result)
}
