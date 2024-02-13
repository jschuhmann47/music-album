use axum::{
    extract::Request,
    http::{header, HeaderMap, StatusCode},
    middleware,
    response::Response,
    Json,
};
use serde_json::Value;

use crate::usecases;

use super::rest;

pub struct UpdateRequest {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub cover: String,
    pub year: i32,
}

pub async fn auth(headers: HeaderMap, request: Request, next: middleware::Next) -> Response {
    let auth_header = match headers.get(header::AUTHORIZATION) {
        Some(t) => t,
        None => {
            return rest::response_body_2(
                StatusCode::BAD_REQUEST,
                String::from("failed to get header"),
            )
        }
    };
    let auth_header = match auth_header.to_str() {
        Ok(str) => str,
        Err(_) => {
            return rest::response_body_2(
                StatusCode::BAD_REQUEST,
                String::from("failed to parse header"),
            )
        }
    };

    let token = match parse_token(auth_header.to_string()) {
        Ok(token) => token,
        Err(err) => {
            return rest::response_body_2(StatusCode::BAD_REQUEST, format!("error: {}", err))
        }
    };

    let res = usecases::auth::execute(token);
    match res {
        Ok(res) => {
            println!("{:?}", res);
            let response = next.run(request).await;
            // println!("{:?}", response);
            response
        }
        Err(_) => rest::response_body_2(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("an error has ocurred"),
        ),
    }
}

fn parse_token(raw_request: String) -> Result<String, String> {
    let mut list = raw_request.split_whitespace();
    let auth_type = match list.next() {
        Some(val) => val,
        None => return Err(String::from("no type")),
    };
    let token = match list.next() {
        Some(val) => val,
        None => return Err(String::from("no value")),
    };

    if !auth_type.eq("Bearer") {
        return Err(String::from("invalid type"));
    }

    Ok(token.to_string())
}
