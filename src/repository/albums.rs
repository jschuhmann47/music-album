use diesel::RunQueryDsl;
use diesel::{Connection, ExpressionMethods, QueryDsl, SelectableHelper};

use crate::schema::albums::id;
use crate::{config, models, schema};

pub struct DbError {
    error: String,
}
impl DbError {
    pub fn new(error: diesel::result::Error) -> DbError {
        DbError {
            error: error.to_string(),
        }
    }
    pub fn get(&self) -> &String {
        &self.error
    }
}

pub fn get(db_conn: config::DbPool, limit: u32) -> Result<Vec<models::Album>, DbError> {
    use self::schema::albums::dsl::*;
    let db = &mut db_conn.get().expect("error getting pool");
    match albums
        .limit(limit.into())
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

pub fn delete(db_conn: config::DbPool, album_id: i32) -> Result<(), DbError> {
    use self::schema::albums;
    // todo define custom errors
    let db = &mut db_conn.get().expect("error getting pool");

    let res = db.transaction(|db| {
        diesel::delete(albums::table)
            .filter(id.eq(album_id))
            .execute(db)
    });
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(DbError::new(err)),
    }
}
