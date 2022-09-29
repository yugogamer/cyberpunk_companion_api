use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Players {
    pub id: i32,
    pub name: String,
    pub family_name: String,
    pub surname: String,
    pub age: i32,
    pub description: String,
    pub image: String,
    pub improvement_points: i32,
    pub used_improvement_points: i32,
    pub created_on: chrono::NaiveDateTime,
    pub owner: i32,
    pub template_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlayers {
    pub name: String,
    pub family_name: String,
    pub surname: Option<String>,
    pub age: Option<i32>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub improvement_points: Option<i32>,
    pub used_improvement_points: Option<i32>,
    pub owner: i32,
    pub template_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletePlayers {
    pub id: i32,
    pub name: String,
    pub family_name: String,
    pub surname: String,
    pub age: i32,
    pub description: String,
    pub image: String,
    pub improvement_points: String,
    pub used_improvement_points: String,
    pub created_on: chrono::NaiveDateTime,
    pub owner: i32,
    pub template: i32,
    pub skills: Vec<super::skills::ComplateSkill>,
    pub stats: Vec<super::stats::Stats>,
}
