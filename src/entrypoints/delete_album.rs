use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::Value;

use super::rest;
use crate::{config, usecases};

// https://docs.rs/axum/latest/axum/#extractors
// van al final!

#[derive(Deserialize)]
pub struct DeleteRequest {
    pub id: i32,
}

pub async fn handler(
    State(db_conn): State<config::DbPool>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<Value>) {
    let res = usecases::delete_album::execute(db_conn, DeleteRequest { id: id });

    match res {
        Ok(res) => (StatusCode::OK, rest::descf(res)),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            rest::descf(e.to_string()),
        ),
    }
}
