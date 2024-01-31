
use diesel::query_dsl::methods::{LimitDsl, SelectDsl};
use diesel::RunQueryDsl;
use diesel::{result::Error, SelectableHelper};

use crate::{config, models};
use crate::schema;



pub fn execute() -> Result<Vec<models::Album>, Error> {
    use self::schema::albums::dsl::*;
    let db = &mut config::connect_to_db();

    albums.limit(5).select(models::Album::as_select()).load(db)
}