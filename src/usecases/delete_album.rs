use crate::{config, entrypoints, repository};

use super::errors::UsecaseError;

pub fn execute(
    db_conn: config::DbPool,
    request: entrypoints::delete_album::DeleteRequest,
) -> Result<(), UsecaseError> {
    if request.id <= 0 {
        return Err(UsecaseError::InvalidID);
    }
    match repository::albums::delete(db_conn, request.user_id, request.id) {
        Ok(res) => Ok(res),
        Err(err) => Err(UsecaseError::DatabaseError(err.get().to_string())),
    }
}
