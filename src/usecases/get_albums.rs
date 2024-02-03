use crate::{config, models, repository};

pub fn execute(
    db_conn: config::DbPool,
    limit: u32,
) -> Result<Vec<models::Album>, diesel::result::Error> {
    repository::albums::get(db_conn, limit)
}
