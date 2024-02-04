use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::Value;

use super::rest;
use crate::{config, usecases};

#[derive(Deserialize)]
pub struct UpdateRequest {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub cover: String,
    pub year: i32,
}

pub async fn handler(
    State(db_conn): State<config::DbPool>,
    Json(req): Json<UpdateRequest>,
) -> (StatusCode, Json<Value>) {
    let res = usecases::update_album::execute(db_conn, req);
    // todo make optional parameters
    match res {
        Ok(res) => (StatusCode::OK, rest::descf(res)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, rest::descf(e)),
    }
}
