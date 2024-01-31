use serde::Serialize;
use serde_json::{json, Value};

pub fn descf<T>(t: T) -> Value 
where
T: Serialize 
{
    json!({"description": t})
}
