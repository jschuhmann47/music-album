pub mod config;
pub mod models;
pub mod schema;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use dotenvy::dotenv;

mod entrypoints {
    pub mod create_album;
    pub mod delete_album;
    pub mod get_albums;
    pub mod rest;
    pub mod login;
    pub mod update_album;
    pub mod auth;
}

mod usecases {
    pub mod create_album;
    pub mod delete_album;
    pub mod errors;
    pub mod get_albums;
    pub mod update_album;
    pub mod login;
    pub mod auth;
}

mod repository {
    pub mod albums;
    pub mod users;
    pub mod errors;
}

mod utils {
    pub mod hash;
    pub mod jwt;
}

#[tokio::main]
async fn main() {
    dotenv().ok();
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
        .route("/", get(entrypoints::login::handler))
        .route("/get", get(entrypoints::get_albums::handler))
        .route("/create", post(entrypoints::create_album::handler))
        .route("/update", put(entrypoints::update_album::handler))
        .route("/delete/:id", delete(entrypoints::delete_album::handler))
        .with_state(db_conn)
}
