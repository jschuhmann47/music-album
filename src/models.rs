use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::albums)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Album {
    pub id: i32,
    pub user_id: i32,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
    pub title: String,
    pub artist: String,
    pub cover: String,
    pub year: i32,
}

impl Album {
    pub fn new(user_id: i32, title: String, artist: String, cover: String, year: i32) -> Album {
        Album {
            id: 0,
            user_id: user_id,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            title,
            artist,
            cover,
            year,
        }
    }
}

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}
