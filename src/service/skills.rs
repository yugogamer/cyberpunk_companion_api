use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::utils::errors::AppErrors;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub types: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSkill {
    player_id: i32,
    skills_template_id: i32,
    value: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTemplate {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub parent_stats_id: Option<i32>,
    pub types: String,
}

impl Skill {
    async fn create_blank_skill(conn: &PgPool, skill: CreateSkill) -> Result<(), AppErrors> {
        match SkillTemplate::get_one_skill_template(conn, skill.skills_template_id).await? {
            Some(parent) => parent,
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
}

impl SkillTemplate {
    async fn get_one_skill_template(
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
}
