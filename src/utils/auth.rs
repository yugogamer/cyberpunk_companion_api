use std::future::{self, Ready};

use crate::entities::account::LightAccount;

use super::{config::Config, errors::AppErrors};
use actix_web::FromRequest;
use jwt::{SignWithKey, VerifyWithKey};

impl FromRequest for LightAccount {
    type Error = AppErrors;
    type Future = Ready<Result<Self, AppErrors>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let config = match req.app_data::<actix_web::web::Data<Config>>() {
            Some(config) => config,
            None => return future::ready(Err(AppErrors::ConfigError)),
        };
        let cookie = req.cookie("session");
        let token = match cookie {
            Some(cookie) => cookie.value().to_string(),
            None => return future::ready(Err(AppErrors::InvalidToken)),
        };

        let token = verify_jwt(&token, &config.jwt_secret);
        match token {
            Ok(light_account) => future::ready(Ok(light_account)),
            Err(_) => future::ready(Err(AppErrors::InvalidToken)),
        }
    }
}

fn generate_random_salt() -> [u8; 32] {
    let mut salt = [0u8; 32];
    for i in salt.iter_mut() {
        *i = rand::random::<u8>();
    }
    salt
}

pub fn password_hash(password: &str, config: &argon2::Config) -> Result<String, AppErrors> {
    let password = password.as_bytes();
    let salt = generate_random_salt();
    let hash = argon2::hash_encoded(password, &salt, config)?;
    Ok(base64::encode(&hash))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppErrors> {
    let password = password.as_bytes();
    let hash = base64::decode(hash)?;
    let hash = std::str::from_utf8(&hash)?;
    let result = argon2::verify_encoded(hash, password)?;
    Ok(result)
}

pub fn generate_jwt(
    accounts: &LightAccount,
    key: &hmac::Hmac<sha2::Sha256>,
) -> Result<String, AppErrors> {
    let token = accounts.sign_with_key(key)?;
    Ok(token)
}

pub fn verify_jwt(token: &str, key: &hmac::Hmac<sha2::Sha256>) -> Result<LightAccount, AppErrors> {
    let accounts: LightAccount = token.verify_with_key(key)?;
    Ok(accounts)
}
