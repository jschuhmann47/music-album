use axum::{
    extract::{Query, State},
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
pub struct GetRequest {
    limit: u32,
}

pub async fn handler(
    State(db_conn): State<config::DbPool>,
    Query(params): Query<GetRequest>,
) -> (StatusCode, Json<Value>) {
    let res = usecases::get_albums::execute(db_conn, params.limit);

    match res {
        Ok(res) => (StatusCode::OK, rest::descf(res)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, rest::descf(e)),
    }
}
