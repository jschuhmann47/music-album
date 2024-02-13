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
    let auth_header = match headers.get(header::AUTHORIZATION) {
        Some(t) => t,
        None => return (StatusCode::INTERNAL_SERVER_ERROR, rest::descf("no auth"))
    };
    let auth_header = match auth_header.to_str() {
        Ok(str) => str,
        Err(_) => return (StatusCode::BAD_REQUEST, rest::descf("failed to parse header"))
    };

    let token = match parse_token(auth_header.to_string()) {
        Ok(token) => token,
        Err(err) => return (StatusCode::BAD_REQUEST, rest::descf(err))
    };

    // (StatusCode::OK, rest::descf(token))

    let res = usecases::auth::execute(token);
    match res {
        Ok(res) => {
            println!("{:?}", res);
            (StatusCode::OK, rest::descf(""))
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, rest::descf(e)),
    }
}

fn parse_token(raw_request: String) -> Result<String, String> {
    let mut list = raw_request.split_whitespace();
    let auth_type = match list.next() {
        Some(val) => val,
        None => return Err(String::from("no type"))
    };
    let token = match list.next() {
        Some(val) => val,
        None => return Err(String::from("no value"))
    };

    if !auth_type.eq("Bearer") {
        return Err(String::from("invalid type"))
    }

    Ok(token.to_string())
    

}
