pub mod config;
pub mod models;
pub mod schema;


use std::vec;

use axum::{http::StatusCode, routing::get, Json, Router};
use diesel::{query_dsl::methods::{LimitDsl, SelectDsl}, RunQueryDsl, SelectableHelper};
use models::Album;

#[tokio::main]
async fn main() {
    
    let app = Router::new().merge(example_routes()); // error is doing app.route for some reason

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> (StatusCode, Json<Album>){
    
    (StatusCode::OK, Json(Album { id: 123, title: String::from("value"), artist: String::from("value"), cover: String::from("value"), year: 1234 }))
}

async fn db_handler() -> (StatusCode, Json<Vec<Album>>){
    use self::schema::albums::dsl::*;
    let db = &mut config::connect_to_db();

    let results = albums.limit(5).select(Album::as_select()).load(db);

    match results {
        Ok(res) => (StatusCode::OK, Json(res)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
    }
    
    // (StatusCode::OK, Json(Album { id: 123, title: String::from("value"), artist: String::from("value"), cover: String::from("value"), year: 1234 }))
}

fn example_routes() -> Router {
    let router = Router::new()
    .route("/", get(handler))
    .route("/db", get(db_handler));
    router
}
