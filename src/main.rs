pub mod config;
pub mod models;


use axum::{http::StatusCode, response::Html, routing::get, Json, Router};
use models::Album;

#[derive(Debug)]
pub enum PostError {
    InternalServerError,
}

#[tokio::main]
async fn main() {
    
    let app = Router::new().route("/", get(handler)); // error is doing app.route for some reason

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> (StatusCode, Json<Album>){
    // let db = &mut config::connect_to_db();
    
    (StatusCode::OK, Json(Album { id: 123, title: String::from("value"), artist: String::from("value"), cover: String::from("value"), year: 1234 }))
}