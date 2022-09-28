use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub types: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSkill {
    pub player_id: i32,
    pub skills_template_id: i32,
    pub value: Option<i32>,
    pub computed_value: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTemplate {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub parent_stats_id: Option<i32>,
    pub types: String,
}

pub struct ComplateSkill {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub parent_stats_id: Option<i32>,
    pub types: String,
    pub computed_value: i32,
    pub value: Option<i32>,
}
