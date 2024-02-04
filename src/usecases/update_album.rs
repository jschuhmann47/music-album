use crate::{
    config, entrypoints,
    models::{self},
    repository,
};

use super::errors::UsecaseError;

pub fn execute(
    db_conn: config::DbPool,
    request: entrypoints::update_album::UpdateRequest,
) -> Result<models::Album, UsecaseError> {
    if request.id <= 0 {
        return Err(UsecaseError::InvalidID);
    }
    if request.title == "" {
        return Err(UsecaseError::InvalidTitle);
    }
    if request.artist == "" {
        return Err(UsecaseError::InvalidArtist);
    }
    if request.cover == "" {
        return Err(UsecaseError::InvalidCover);
    }
    if request.year <= 0 {
        return Err(UsecaseError::InvalidYear);
    }
    let album = models::Album {
        id: request.id,
        title: request.title,
        artist: request.artist,
        cover: request.cover,
        year: request.year,
    };

    match repository::albums::update(db_conn, album) {
        Ok(res) => Ok(res),
        Err(err) => Err(UsecaseError::DatabaseError(err.get().to_string())),
    }
}
