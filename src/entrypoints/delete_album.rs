use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

use super::rest;
use crate::{config, usecases};

#[derive(Deserialize)]
pub struct DeleteRequest {
    pub user_id: i32,
    pub id: i32,
}

pub async fn handler(
    State(db_conn): State<config::DbPool>,
    Extension(user_id): Extension<i32>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<Value>) {
    let req = DeleteRequest {
        user_id: user_id,
        id: id,
    };
    let res = usecases::delete_album::execute(db_conn, req);

    match res {
        Ok(_) => (StatusCode::OK, Json(json!({}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, rest::descf(e)),
    }
}
