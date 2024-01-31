use diesel::result::Error;

use crate::{models, repository};

pub fn execute() -> Result<Vec<models::Album>, Error> {
    repository::get_albums::get()
}