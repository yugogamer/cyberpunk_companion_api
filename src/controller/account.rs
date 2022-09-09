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
    service::account::{Account, CreateAccount, LightAccount, Login},
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

    let token = Account::auth_account(&conn, username, password, &config.jwt_secret).await?;

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
    Account::create_account(&conn, create, &config.argon2_config).await?;
    Ok(HttpResponse::new(StatusCode::OK))
}

#[get("/info")]
async fn get_account(account: LightAccount) -> Result<Json<LightAccount>, AppErrors> {
    Ok(Json(account))
}
