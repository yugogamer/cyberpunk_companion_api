use std::future::Ready;

use crate::service::account::LightAccount;

use super::errors::AppErrors;
use actix_web::FromRequest;
use jwt::{SignWithKey, VerifyWithKey};

pub struct User;

impl FromRequest for User {
    type Error = AppErrors;
    type Future = Ready<Result<User, AppErrors>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        todo!()
    }
}

fn generate_random_salt() -> [u8; 32] {
    let mut salt = [0u8; 32];
    for i in 0..32 {
        salt[i] = rand::random::<u8>();
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

pub fn verify_jwt(token: &str, key: hmac::Hmac<sha2::Sha256>) -> Result<LightAccount, AppErrors> {
    let accounts: LightAccount = token.verify_with_key(&key)?;
    Ok(accounts)
}
