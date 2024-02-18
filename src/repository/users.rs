use diesel::{Connection, RunQueryDsl};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};

use crate::models::User;
use crate::{config, models};

use super::errors::DbError;

pub fn get_by_username(db_conn: &config::DbPool, usrname: &String) -> Result<models::User, DbError> {
    use crate::schema::users::dsl::*;
    let db = &mut db_conn.get().expect("error getting pool");
    match users
        .filter(username.eq(usrname))
        .select(models::User::as_select())
        .first(db)
    {
        Ok(res) => Ok(res),
        Err(err) => match err {
            diesel::result::Error::NotFound => Ok(User {
                id: 0,
                username: String::from(""),
                password: String::from(""),
            }),
            err => Err(DbError::new(err)),
        },
    }
}

pub fn create(db_conn: &config::DbPool, user: models::User) -> Result<models::User, DbError> {
    use crate::schema::users;
    let db = &mut db_conn.get().expect("error getting pool");

    let res = db.transaction(|db| {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(db)?;
        users::table
            .order(users::id.desc())
            .select(models::User::as_select())
            .first(db)
    });
    match res {
        Ok(res) => Ok(res),
        Err(err) => Err(DbError::new(err)),
    }
}
