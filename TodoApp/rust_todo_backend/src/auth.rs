use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use super::config::JWT_SECRET;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: usize,
}

impl Claims {
    pub fn new(user_id: &str, ttl_minutes: u64) -> Self {
        let exp = SystemTime::now()
            .checked_add(Duration::from_secs(ttl_minutes * 60))
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        Claims {
            sub: user_id.to_string(),
            exp,
        }
    }
}

pub fn generate_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(user_id, 60); // 1 hour
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&*JWT_SECRET),
    )
}

pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(&*JWT_SECRET),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
