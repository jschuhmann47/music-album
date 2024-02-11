
use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Error as JWTError, TokenData};
use serde::{Deserialize, Serialize};


// https://github.com/Keats/jsonwebtoken/blob/master/examples/validation.rs

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: u32, // subject, can use user id
    exp: u64, // unix expiration date
}

fn new_claim(user_id: u32) -> Claims {
    Claims{sub: 1, exp: time_now()}
}

fn generate_token() -> Result<String, JWTError>{
    encode(&Header::default(), &new_claim(1), &EncodingKey::from_secret("secret".as_ref()))
}

fn decode_token(token: String) -> Result<TokenData<Claims>, JWTError> {
    decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default())
}

fn time_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}