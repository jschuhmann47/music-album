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

pub fn response_body<T>(status: StatusCode, desc: T) -> Response<Body> 
where T: Serialize,
{
    Response::builder()
    .status(status)
    .body(Body::from(json!({
        "description": desc
    }).to_string()))
    .unwrap()
}
