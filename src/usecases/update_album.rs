use crate::{
    config, entrypoints,
    models::{self},
    repository,
};

pub fn execute(
    db_conn: config::DbPool,
    request: entrypoints::update_album::UpdateRequest,
) -> Result<models::Album, diesel::result::Error> {
    // TODO validate
    let album = models::Album {
        id: request.id,
        title: request.title,
        artist: request.artist,
        cover: request.cover,
        year: request.year,
    };

    repository::albums::update(db_conn, album)
}
