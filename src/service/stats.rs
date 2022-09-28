use sqlx::PgPool;

use crate::{entities::stats::*, utils::errors::AppErrors};

pub async fn create_stats_template(conn: &PgPool, stats: CreateStats) -> Result<(), AppErrors> {
    let computed_value = stats.base_value + stats.modifier;
    sqlx::query!(
        "INSERT INTO stats (player_id, stats_template_id, base_value, modifier, computed_value) VALUES ($1, $2, $3, $4, $5)",
        stats.player_id,
        stats.stats_template_id,
        stats.base_value,
        stats.modifier,
        computed_value,
    )
    .execute(conn)
    .await?;
    Ok(())
}

pub async fn get_one_stats_by_user(
    conn: &PgPool,
    template_id: i32,
    player_id: i32,
) -> Result<Option<Stats>, AppErrors> {
    let result = sqlx::query_as!(
        Stats,
        "SELECT s.player_id, s.stats_template_id, st.name, st.description, t.name as types, s.base_value, s.modifier, s.computed_value
            FROM stats as s
            inner join stats_template as st on st.id = s.stats_template_id
            inner join \"types\" as t on t.id = st.type
            where s.player_id = $1
            AND s.stats_template_id = $2",
        player_id,
        template_id
    )
    .fetch_optional(conn)
    .await?;
    Ok(result)
}
