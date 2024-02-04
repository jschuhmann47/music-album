use crate::{
    config, entrypoints,
    models::{self},
    repository,
};

use super::errors::UsecaseError;

pub fn execute(
    db_conn: config::DbPool,
    request: entrypoints::create_album::CreateRequest,
) -> Result<models::Album, UsecaseError> {
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
        id: 0,
        title: request.title,
        artist: request.artist,
        cover: request.cover,
        year: request.year,
    };

    match repository::albums::create(db_conn, album) {
        Ok(res) => Ok(res),
        Err(err) => Err(UsecaseError::DatabaseError(err.get().to_string())),
    }
}
