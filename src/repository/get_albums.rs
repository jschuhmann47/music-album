use diesel::{query_dsl::methods::SelectDsl, RunQueryDsl, SelectableHelper};

use crate::{config, models, schema};

pub fn get(db_conn: config::DbPool) -> Result<Vec<models::Album>, diesel::result::Error> {
    use self::schema::albums::dsl::*;
    // todo define custom errors
    let db = &mut db_conn.get().expect("error getting pool");
    albums.select(models::Album::as_select()).load(db)
}
