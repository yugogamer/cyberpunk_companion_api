use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsTemplate {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub types: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStatsTemplate {
    pub name: String,
    pub description: Option<String>,
    pub types: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub player_id: i32,
    pub stats_template_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub types: String,
    pub base_value: i32,
    pub modifier: i32,
    pub computed_value: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStats {
    pub player_id: i32,
    pub stats_template_id: i32,
    pub base_value: i32,
    pub modifier: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStats {
    pub id: i32,
    pub base_value: i32,
    pub modifier: i32,
}
