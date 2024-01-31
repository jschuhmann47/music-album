use axum::{http::StatusCode, Json};
use serde_json::Value;

use crate::models;
use super::rest;


pub async fn handler() -> (StatusCode, Json<Value>) {
    let album = models::Album { id: 123, title: String::from("value"), artist: String::from("value"), cover: String::from("value"), year: 1234 };
    let res = rest::descf(album);
    (StatusCode::OK, Json(res))
}
