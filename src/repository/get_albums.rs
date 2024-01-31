use diesel::{query_dsl::methods::SelectDsl, result::Error, RunQueryDsl, SelectableHelper};

use crate::{config, models, schema};

pub fn get() -> Result<Vec<models::Album>, Error> {
    use self::schema::albums::dsl::*;
    let db = &mut config::connect_to_db();
    albums.select(models::Album::as_select()).load(db)
}