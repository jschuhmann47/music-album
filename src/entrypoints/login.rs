use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::Value;

use super::rest;
use crate::{config, usecases};

// https://docs.rs/axum/latest/axum/#extractors
// van al final!

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn handler(
    State(db_conn): State<config::DbPool>,
    Json(req): Json<LoginRequest>,
) -> (StatusCode, Json<Value>) {
    let res = usecases::login::execute(db_conn, req.username, req.password);

    match res {
        Ok(res) => (StatusCode::OK, rest::descf(res)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, rest::descf(e)),
    }
}
