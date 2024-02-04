use serde::Serialize;

#[derive(Serialize)]
pub enum UsecaseError {
    InvalidTitle,
    InvalidCover,
    InvalidYear,
    InvalidID,
    InvalidArtist,
    InvalidLimit,
    DatabaseError(String),
}
