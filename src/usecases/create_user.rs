use crate::{
    config, entrypoints,
    models,
    repository, utils,
};

use super::errors::UsecaseError;

pub fn execute(
    db_conn: config::DbPool,
    request: entrypoints::create_user::CreateUserRequest,
) -> Result<(), UsecaseError> {
    if request.username == "" {
        return Err(UsecaseError::InvalidUsernameOrPassword);
    }
    if request.password == "" {
        return Err(UsecaseError::InvalidUsernameOrPassword);
    }

    let existing_user = match repository::users::get_by_username(&db_conn, &request.username){
        Ok(usr) => usr,
        Err(err) => return Err(UsecaseError::DatabaseError(err.get().to_string())),
    };
    if existing_user.username.eq(&request.username) {
        return Err(UsecaseError::UsernameAlreadyInUse)
    }

    let user = models::User{
        id: 0,
        username: request.username,
        password: utils::hash::sha256(request.password),
    };

    match repository::users::create(&db_conn, user) {
        Ok(_) => Ok(()),
        Err(err) => Err(UsecaseError::DatabaseError(err.get().to_string())),
    }
}
