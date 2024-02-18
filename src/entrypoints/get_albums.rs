use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use serde::Deserialize;
use serde_json::Value;

use super::rest;
use crate::{config, usecases};

#[derive(Deserialize)]
pub struct GetRequest {
    limit: u32,
}

pub async fn handler(
    State(db_conn): State<config::DbPool>,
    Extension(user_id): Extension<i32>,
    Query(params): Query<GetRequest>,
) -> (StatusCode, Json<Value>) {
    let res = usecases::get_albums::execute(db_conn, user_id, params.limit);

    match res {
        Ok(res) => (StatusCode::OK, rest::descf(res)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, rest::descf(e)),
    }
}
