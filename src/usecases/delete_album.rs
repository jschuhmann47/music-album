use crate::{config, entrypoints, repository};

pub fn execute(
    db_conn: config::DbPool,
    request: entrypoints::delete_album::DeleteRequest,
) -> Result<(), diesel::result::Error> {
    repository::albums::delete(db_conn, request.id)
}
