use axum::{http::StatusCode, Json};
use serde_json::Value;

use crate::usecases;
use super::rest;

pub async fn handler() -> (StatusCode, Json<Value>){
    let res = usecases::db::execute();
    
    match res {
        Ok(res) => (StatusCode::OK, rest::descf(res)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, rest::descf(e.to_string()))
    }
}