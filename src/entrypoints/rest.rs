use axum::Json;
use serde::Serialize;
use serde_json::{json, Value};

pub fn descf<T>(t: T) -> Json<Value>
where
    T: Serialize,
{
    Json(json!({"description": t}))
}

pub struct ResponseBody {
    description: Json<Value>,
}
