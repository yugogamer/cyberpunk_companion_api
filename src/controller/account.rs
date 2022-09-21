use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self, Json},
    HttpResponse,
};
use cookie::Cookie;
use sqlx::{Pool, Postgres};

use crate::{
    entities::account::{Account, CreateAccount, LightAccount, Login, PublicAccount},
    service::account::{auth_account, create_account, get_public_account},
    utils::{config::Config, errors::AppErrors},
};

#[post("/login")]
async fn login(
    conn: web::Data<Pool<Postgres>>,
    config: web::Data<Config>,
    login: web::Json<Login>,
) -> Result<HttpResponse, AppErrors> {
    let login = login.into_inner();
    let username = login.username;
    let password = login.password;

    let token = auth_account(&conn, username, password, &config.jwt_secret).await?;

    let cookie = Cookie::new("session", token);

    let mut result = HttpResponse::new(StatusCode::OK);
    result.add_cookie(&cookie)?;
    Ok(result)
}

#[post("/register")]
async fn register(
    conn: web::Data<Pool<Postgres>>,
    config: web::Data<Config>,
    create: web::Json<CreateAccount>,
) -> Result<HttpResponse, AppErrors> {
    let create = create.into_inner();
    create_account(&conn, create, &config.argon2_config).await?;
    Ok(HttpResponse::new(StatusCode::OK))
}

#[get("/info")]
async fn get_account(
    conn: web::Data<Pool<Postgres>>,
    account: LightAccount,
) -> Result<Json<PublicAccount>, AppErrors> {
    Ok(Json(get_public_account(&conn, account.id).await?))
}
