use jsonwebtoken::TokenData;

use crate::utils::jwt;

pub enum AuthError {
    InvalidToken,
}

pub fn execute(token: String) -> Result<TokenData<jwt::Claims>, AuthError> {
   
    let token_data = match jwt::decode_token(token) {
        Ok(tk) => tk,
        Err(_) => return Err(AuthError::InvalidToken)
    };

    Ok(token_data)
}