pub mod config;
pub mod models;
pub mod schema;

use axum::{
    routing::{get, post, put},
    Router,
};

mod entrypoints {
    pub mod create_album;
    pub mod get_albums;
    pub mod update_album;
    pub mod rest;
    pub mod test;
}

mod usecases {
    pub mod create_album;
    pub mod get_albums;
    pub mod update_album;
    pub mod test;
}

mod repository {
    pub mod albums;
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
        .route("/", get(entrypoints::test::handler))
        .route("/get", get(entrypoints::get_albums::handler))
        .route("/create", post(entrypoints::create_album::handler))
        .route("/update", put(entrypoints::update_album::handler))
        .with_state(db_conn)
}
