use axum::{http::StatusCode, Json};
use serde_json::Value;

use super::rest;
use crate::usecases;

pub async fn handler() -> (StatusCode, Json<Value>) {
    let album = usecases::test::execute();
    (StatusCode::OK, rest::descf(album))
}
