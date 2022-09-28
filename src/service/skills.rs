use sqlx::PgPool;

use super::{
    skills_template::{self, get_one_skill_template},
    stats::get_one_stats_by_user,
};
use crate::{
    entities::skills::{CreateSkill, SkillTemplate},
    utils::errors::AppErrors,
};

async fn create_blank_skill(conn: &PgPool, mut skill: CreateSkill) -> Result<(), AppErrors> {
    match get_one_stats_by_user(conn, skill.skills_template_id, skill.player_id).await? {
        Some(parent) => {
            skill.computed_value =
                Some(parent.computed_value.unwrap_or(0) + skill.value.unwrap_or(0));
        }
        None => return Err(AppErrors::NotFound("Parent not found".to_string())),
    };

    sqlx::query!(
        "INSERT INTO skills (player_id, skill_template_id, value) VALUES ($1, $2, $3)",
        skill.player_id,
        skill.skills_template_id,
        skill.value
    )
    .execute(conn)
    .await?;
    Ok(())
}

pub async fn get_skills_for_character(
    conn: &PgPool,
    player_id: i32,
) -> Result<Vec<SkillTemplate>, AppErrors> {
    let result = sqlx::query_as!(
        SkillTemplate,
        "SELECT st.id, st.name, st.description, st.parent_stats_id, t.name as types
            FROM skills AS s
            inner join skills_template as st on st.id = s.skill_template_id
            inner join \"types\" as t on t.id = st.type
            where s.player_id = $1",
        player_id,
    )
    .fetch_all(conn)
    .await?;
    Ok(result)
}
