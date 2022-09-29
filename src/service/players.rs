use sqlx::PgPool;

use crate::{entities::players::*, utils::errors::AppErrors};

pub async fn create_player(conn: &PgPool, players: CreatePlayers) {
    let result = sqlx::query_as!(
        Players,
        "INSERT INTO players(name,family_name,surname,age,description,image, improvement_points, used_improvement_points, owner, template_id)
        VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
        returning *",
        players.name,
        players.family_name,
        players.surname,
        players.age,
        players.description,
        players.image,
        players.improvement_points,
        players.used_improvement_points,
        players.owner,
        players.template_id
    );
}
