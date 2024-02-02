use crate::models;

pub fn execute() -> models::Album {
    models::Album {
        id: 1,
        title: String::from("value"),
        artist: String::from("value"),
        cover: String::from("value"),
        year: 123,
    }
}
