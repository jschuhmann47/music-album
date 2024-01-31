pub mod config;
pub mod models;
pub mod schema;

use axum::{routing::get, Router};

mod entrypoints {
    pub mod example;
    pub mod db_example;
    pub mod rest;
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
    .route("/", get(entrypoints::example::handler))
    .route("/db", get(entrypoints::db_example::handler));
    router
}
