use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::albums)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Album {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub cover: String,
    pub year: i32,
}

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}
