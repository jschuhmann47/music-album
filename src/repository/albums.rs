use diesel::RunQueryDsl;
use diesel::{Connection, ExpressionMethods, QueryDsl, SelectableHelper};

use crate::{config, models, schema};

pub fn get(
    db_conn: config::DbPool,
    limit: u32,
) -> Result<Vec<models::Album>, diesel::result::Error> {
    use self::schema::albums::dsl::*;
    // todo define custom errors
    let db = &mut db_conn.get().expect("error getting pool");
    albums
        .limit(limit.into())
        .select(models::Album::as_select())
        .load(db)
}

pub fn create(
    db_conn: config::DbPool,
    album: models::Album,
) -> Result<models::Album, diesel::result::Error> {
    use self::schema::albums;
    let db = &mut db_conn.get().expect("error getting pool");

    let res = db.transaction(|db| {
        diesel::insert_into(albums::table)
            .values(&album)
            .execute(db)?;
        albums::table
            .order(albums::id.desc())
            .select(models::Album::as_select())
            .first(db)
    });
    match res {
        Ok(res) => Ok(res),
        Err(_) => Err(diesel::result::Error::AlreadyInTransaction), //change this err
    }
}
