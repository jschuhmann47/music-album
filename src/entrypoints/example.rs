use axum::{http::StatusCode, Json};

use crate::models;


pub async fn handler() -> (StatusCode, Json<models::Album>){
    
    (StatusCode::OK, Json(models::Album { id: 123, title: String::from("value"), artist: String::from("value"), cover: String::from("value"), year: 1234 }))
}
