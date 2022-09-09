use argon2::Config;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};

use crate::utils::{self, errors::AppErrors};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    id: i32,
    username: String,
    email: String,
    password: String,
    created_on: chrono::NaiveDateTime,
    last_login: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicAccount {
    id: i32,
    username: String,
    created_on: chrono::NaiveDateTime,
    last_login: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccount {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightAccount {
    pub id: i32,
    pub username: String,
}

impl Account {
    pub async fn create_account(
        conn: &PgPool,
        create_account: CreateAccount,
        config: &Config<'_>,
    ) -> Result<(), AppErrors> {
        let result: Vec<Account> = sqlx::query_as!(
            Account,
            "SELECT * FROM accounts
            where username = $1 or email = $2",
            create_account.username,
            create_account.email,
        )
        .fetch_all(conn)
        .await?;

        if !result.is_empty() {
            return Err(AppErrors::EmailAlreadyUsed());
        }

        let hash = utils::auth::password_hash(&create_account.password, config)?;

        sqlx::query!(
            "INSERT INTO accounts (username, email, password) VALUES ($1, $2, $3)",
            create_account.username,
            create_account.email,
            hash
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn auth_account(
        conn: &PgPool,
        username: String,
        password: String,
        key: &hmac::Hmac<sha2::Sha256>,
    ) -> Result<String, AppErrors> {
        let password_db: Account = query_as!(
            Account,
            "SELECT * FROM accounts WHERE username = $1",
            username
        )
        .fetch_one(conn)
        .await?;

        if !utils::auth::verify_password(&password, &password_db.password)? {
            return Err(AppErrors::NoUserFinde);
        }

        sqlx::query!(
            "UPDATE accounts SET last_login = $1 WHERE username = $2",
            chrono::Utc::now().naive_utc(),
            username
        )
        .execute(conn)
        .await?;

        let light_account = LightAccount {
            id: password_db.id,
            username: password_db.username,
        };

        let token = utils::auth::generate_jwt(&light_account, key)?;

        Ok(token)
    }
}

impl PublicAccount {
    pub async fn get_public_account(conn: &PgPool, id: i32) -> Result<PublicAccount, AppErrors> {
        let result: PublicAccount = query_as!(
            PublicAccount,
            "SELECT id, username, created_on, last_login FROM accounts WHERE id = $1",
            id
        )
        .fetch_one(conn)
        .await?;

        Ok(result)
    }
}
