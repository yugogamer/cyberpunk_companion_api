use sqlx::PgPool;

use crate::{entities::players::*, utils::errors::AppErrors};

pub async fn create_player(conn: &PgPool, players: CreatePlayers) -> Result<Players, AppErrors> {
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
    ).fetch_optional(conn).await?;
    match result {
        Some(player) => Ok(player),
        None => Err(AppErrors::ConfigError),
    }
}

pub async fn liste_players_by_account(
    conn: &PgPool,
    id_account: i32,
) -> Result<Vec<Players>, AppErrors> {
    Ok(sqlx::query_as!(
        Players,
        "SELECT *
        from players where owner = $1",
        id_account,
    )
    .fetch_all(conn)
    .await?)
}

pub async fn liste_players_by_groupes(
    conn: &PgPool,
    id_groupes: i32,
) -> Result<Vec<Players>, AppErrors> {
    Ok(sqlx::query_as!(
        Players,
        "select pl.*
        from players as pl
        inner join players_to_groupes as ptg on ptg.player_id = pl.id
        where ptg.groupe_id = $1
        ",
        id_groupes
    )
    .fetch_all(conn)
    .await?)
}
