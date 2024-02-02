pub mod config;
pub mod models;
pub mod schema;

use axum::{routing::get, Router};

mod entrypoints {
    pub mod db_example;
    pub mod rest;
    pub mod test_album;
}

mod usecases {
    pub mod db;
    pub mod test;
}

mod repository {
    pub mod get_albums;
}

#[tokio::main]
async fn main() {
    let app = Router::new().merge(example_routes());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn example_routes() -> Router {
    let db_conn = config::get_connection_pool();

    Router::new()
        .route("/", get(entrypoints::test_album::handler))
        .route("/db", get(entrypoints::db_example::handler))
        .with_state(db_conn)
}
