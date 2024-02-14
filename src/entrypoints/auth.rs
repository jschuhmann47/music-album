use axum::{
    extract::Request,
    http::{header, HeaderMap, StatusCode},
    middleware,
    response::Response,
};

use crate::usecases;

use super::rest;

pub async fn auth(headers: HeaderMap, request: Request, next: middleware::Next) -> Response {
    let auth_header = match headers.get(header::AUTHORIZATION) {
        Some(t) => t,
        None => {
            return rest::response_body(
                StatusCode::BAD_REQUEST,
                String::from("failed to get header"),
            )
        }
    };
    let auth_header = match auth_header.to_str() {
        Ok(str) => str,
        Err(_) => {
            return rest::response_body(
                StatusCode::BAD_REQUEST,
                String::from("failed to parse header"),
            )
        }
    };

    let token = match parse_token(auth_header.to_string()) {
        Ok(token) => token,
        Err(_) => {
            return rest::response_body(
                StatusCode::BAD_REQUEST,
                String::from("failed to parse token"),
            )
        }
    };

    let res = usecases::auth::execute(token);
    match res {
        Ok(_) => {
            let response = next.run(request).await;
            response
        }
        Err(err) => rest::response_body(StatusCode::INTERNAL_SERVER_ERROR, err),
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
