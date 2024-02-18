use diesel::RunQueryDsl;
use diesel::{Connection, ExpressionMethods, QueryDsl, SelectableHelper};

use crate::schema::albums::{id, user_id};
use crate::{config, models, schema};

use super::errors::DbError;

pub fn get(
    db_conn: config::DbPool,
    user_id_filter: i32,
    limit: u32,
) -> Result<Vec<models::Album>, DbError> {
    use self::schema::albums::dsl::*;
    let db = &mut db_conn.get().expect("error getting pool");
    match albums
        .limit(limit.into())
        .filter(user_id.eq(user_id_filter))
        .order(id.desc())
        .select(models::Album::as_select())
        .load(db)
    {
        Ok(res) => Ok(res),
        Err(err) => Err(DbError::new(err)),
    }
}

pub fn create(db_conn: config::DbPool, album: models::Album) -> Result<models::Album, DbError> {
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
        Err(err) => Err(DbError::new(err)),
    }
}

pub fn update(db_conn: config::DbPool, album: models::Album) -> Result<models::Album, DbError> {
    use self::schema::albums;
    let db = &mut db_conn.get().expect("error getting pool");

    let res = db.transaction(|db| {
        diesel::update(albums::table)
            .filter(id.eq(album.id))
            .set(&album)
            .execute(db)?;
        albums::table
            .order(albums::id.desc())
            .select(models::Album::as_select())
            .first(db)
    });
    match res {
        Ok(res) => Ok(res),
        Err(err) => Err(DbError::new(err)),
    }
}

pub fn delete(db_conn: config::DbPool, user_id_filter: i32, album_id: i32) -> Result<(), DbError> {
    use self::schema::albums;
    // todo define custom errors
    let db = &mut db_conn.get().expect("error getting pool");

    let res = db.transaction(|db| {
        diesel::delete(albums::table)
            .filter(id.eq(album_id))
            .filter(user_id.eq(user_id_filter))
            .execute(db)
    });
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(DbError::new(err)),
    }
}
