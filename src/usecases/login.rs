use serde::Serialize;

use crate::{config, repository, utils};

use super::errors::UsecaseError;

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

pub fn execute(
    db_conn: config::DbPool,
    username: String,
    password: String,
) -> Result<LoginResponse, UsecaseError> {
    if username == "" || password == "" {
        return Err(UsecaseError::InvalidUsernameOrPassword);
    }
    let hashed_pass = utils::hash::sha256(password);

    let user = match repository::users::get_by_username(&db_conn, &username) {
        Ok(res) => res,
        Err(err) => return Err(UsecaseError::DatabaseError(err.get().to_string())),
    };
    if user.id == 0 {
        return Err(UsecaseError::InvalidUsernameOrPassword);
    }
    if !user.password.eq(&hashed_pass) {
        return Err(UsecaseError::InvalidUsernameOrPassword);
    }

    match utils::jwt::generate_token(user.id) {
        Ok(token) => Ok(LoginResponse { token }),
        Err(_) => Err(UsecaseError::ErrorGeneratingToken),
    }
}
