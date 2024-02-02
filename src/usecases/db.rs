use crate::{config, models, repository};

pub fn execute(db_conn: config::DbPool) -> Result<Vec<models::Album>, diesel::result::Error> {
    repository::get_albums::get(db_conn)
}
