use crate::{config, models, repository};

use super::errors::UsecaseError;

pub fn execute(db_conn: config::DbPool, limit: u32) -> Result<Vec<models::Album>, UsecaseError> {
    if limit <= 0 {
        return Err(UsecaseError::InvalidLimit);
    }

    match repository::albums::get(db_conn, limit) {
        Ok(res) => Ok(res),
        Err(err) => Err(UsecaseError::DatabaseError(err.get().to_string())),
    }
}
