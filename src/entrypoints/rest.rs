use axum::{
    body::Body,
    http::{Response, StatusCode},
    Json,
};
use serde::Serialize;
use serde_json::{json, Value};

pub fn descf<T>(t: T) -> Json<Value>
where
    T: Serialize,
{
    Json(json!({"description": t}))
}

struct DescResponse{
    description: String
}

pub fn response_body_2(status: StatusCode, desc: String) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(Body::new(desc))
        .unwrap()
}
