pub mod config;
pub mod models;
pub mod schema;

use axum::{routing::get, Router};

mod entrypoints {
    pub mod test_album;
    pub mod db_example;
    pub mod rest;
}

mod usecases {
    pub mod test;
    pub mod db;
}

#[tokio::main]
async fn main() {
    
    let app = Router::new().merge(example_routes());

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn example_routes() -> Router {
    let router = Router::new()
    .route("/", get(entrypoints::test_album::handler))
    .route("/db", get(entrypoints::db_example::handler));
    router
}
