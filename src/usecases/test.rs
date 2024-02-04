use crate::models;

pub fn execute() -> models::Album {
    models::Album {
        id: 1,
        title: String::from("title test"),
        artist: String::from("artist test"),
        cover: String::from("cover test"),
        year: 123,
    }
}
