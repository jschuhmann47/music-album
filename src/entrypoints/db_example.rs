use axum::{http::StatusCode, Json};
use diesel::query_dsl::methods::{LimitDsl, SelectDsl};
use diesel::{RunQueryDsl, SelectableHelper};

use crate::{config, models};
use crate::schema;



pub async fn handler() -> (StatusCode, Json<Vec<models::Album>>){
    use self::schema::albums::dsl::*;
    let db = &mut config::connect_to_db();

    let results = albums.limit(5).select(models::Album::as_select()).load(db);

    match results {
        Ok(res) => (StatusCode::OK, Json(res)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
    }
}