use serde::Serialize;

#[derive(Serialize)]
pub struct Album {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub cover: String,
    pub year: i32,
}