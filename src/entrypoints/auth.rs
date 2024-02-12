use axum::{extract::State, http::{header, HeaderMap, StatusCode}, Json};
use serde::Deserialize;
use serde_json::Value;

use super::rest;
use crate::{config, usecases};


#[derive(Deserialize)]
pub struct UpdateRequest {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub cover: String,
    pub year: i32,
}

pub async fn handler(headers: HeaderMap) -> (StatusCode, Json<Value>) {
    // get token from header
    let user_agent = match headers.get(header::AUTHORIZATION) {
        Some(t) => t,
        None => return (StatusCode::INTERNAL_SERVER_ERROR, rest::descf("no auth"))
    };
    let token = parse_token(user_agent);

    let res = usecases::auth::execute();
    // todo make optional parameters
    match res {
        Ok(res) => (StatusCode::OK, rest::descf(res)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, rest::descf(e)),
    }
}

fn parse_token(raw_request: String) -> (String, String) {
    let list: Vec<&str> = raw_request.split_whitespace().collect();
    (list.get)
}
