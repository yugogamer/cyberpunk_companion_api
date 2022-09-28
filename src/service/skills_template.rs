use sqlx::PgPool;

use crate::{
    entities::skills::{CreateSkill, SkillTemplate},
    utils::errors::AppErrors,
};

pub async fn get_one_skill_template(
    conn: &PgPool,
    id: i32,
) -> Result<Option<SkillTemplate>, AppErrors> {
    let result = sqlx::query_as!(
        SkillTemplate,
        "SELECT st.id, st.name, st.description, st.parent_stats_id, t.name as types
            FROM skills_template AS st
            inner join \"types\" as t on t.id = st.type
            where st.id = $1",
        id,
    )
    .fetch_optional(conn)
    .await?;
    Ok(result)
}

pub async fn get_all_skill_template(
    conn: &PgPool,
    limite: i64,
    offset: i64,
) -> Result<Vec<SkillTemplate>, AppErrors> {
    let result = sqlx::query_as!(
        SkillTemplate,
        "SELECT st.id, st.name, st.description, st.parent_stats_id, t.name as types
            FROM skills_template AS st
            inner join \"types\" as t on t.id = st.type
            LIMIT $1 OFFSET $2",
        limite,
        offset,
    )
    .fetch_all(conn)
    .await?;
    Ok(result)
}
