use axum::{extract::State, http::StatusCode, Json};
use serde_json::Value;

use super::rest;
use crate::{config, usecases};

pub async fn handler(State(db_conn): State<config::DbPool>) -> (StatusCode, Json<Value>) {
    let res = usecases::db::execute(db_conn);

    match res {
        Ok(res) => (StatusCode::OK, rest::descf(res)),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            rest::descf(e.to_string()),
        ),
    }
}
