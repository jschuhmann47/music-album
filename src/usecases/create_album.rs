use crate::{
    config, entrypoints,
    models::{self},
    repository,
};

pub fn execute(
    db_conn: config::DbPool,
    request: entrypoints::create_album::CreateRequest,
) -> Result<models::Album, diesel::result::Error> {
    let album = models::Album {
        id: 0,
        title: request.title,
        artist: request.artist,
        cover: request.cover,
        year: request.year,
    };

    repository::albums::create(db_conn, album)
}
