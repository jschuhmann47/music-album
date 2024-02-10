
use base16ct::Error;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, errors::Error as JWTError, TokenData};
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // subject, can use user id
    exp: usize, // unix expiration date
}

fn new_claim() -> Claims {
    Claims{sub: String::from("1"), exp: 1739197106}
}

fn generate_token() -> Result<String, JWTError>{
    encode(&Header::default(), &new_claim(), &EncodingKey::from_secret("secret".as_ref()))
}

fn decode_token(token: String) -> Result<TokenData<Claims>, JWTError> {
    decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default())
}