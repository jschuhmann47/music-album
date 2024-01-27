use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::albums)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Album {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub cover: String,
    pub year: i32,
}