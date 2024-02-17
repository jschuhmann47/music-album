use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::albums)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Album {
    pub id: i32,
    pub user_id: i32,
    created_at: chrono::NaiveDateTime,
    updated_at: Option<chrono::NaiveDateTime>,
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
