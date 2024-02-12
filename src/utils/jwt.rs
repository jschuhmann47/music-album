
use std::{env, ops::Add, time::{SystemTime, UNIX_EPOCH}};

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Error as JWTError, TokenData};
use serde::{Deserialize, Serialize};



// https://github.com/Keats/jsonwebtoken/blob/master/examples/validation.rs

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: u32, // subject, can use user id
    pub exp: u64, // unix expiration date
}

fn new_claim(user_id: u32) -> Claims {
    Claims{sub: user_id, exp: time_now()}
}

pub fn generate_token(user_id: u32) -> Result<String, JWTError>{
    // todo don't load from env everytime
    let secret = env::var("JWT_SECRET").expect("failed to load jwt secret");
    encode(&Header::default(), &new_claim(user_id), &EncodingKey::from_secret(secret.as_ref()))
}

pub fn decode_token(token: String) -> Result<TokenData<Claims>, JWTError> {
    let secret = env::var("JWT_SECRET").expect("failed to load jwt secret");
    decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())
}

fn time_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().add(10000)
}
