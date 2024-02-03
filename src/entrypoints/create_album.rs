use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::Value;

use super::rest;
use crate::{config, usecases};

// https://docs.rs/axum/latest/axum/#extractors
// van al final!

#[derive(Deserialize)]
pub struct CreateRequest {
    pub title: String,
    pub artist: String,
    pub cover: String,
    pub year: i32,
}

pub async fn handler(
    State(db_conn): State<config::DbPool>,
    Json(req): Json<CreateRequest>,
) -> (StatusCode, Json<Value>) {
    let res = usecases::create_album::execute(db_conn, req);

    match res {
        Ok(res) => (StatusCode::OK, rest::descf(res)),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            rest::descf(e.to_string()),
        ),
    }
}
