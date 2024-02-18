pub mod config;
pub mod models;
pub mod schema;

use axum::{
    middleware::from_fn,
    routing::{delete, get, post, put},
    Router,
};
use config::DbPool;
use dotenvy::dotenv;

mod entrypoints {
    pub mod auth;
    pub mod create_album;
    pub mod create_user;
    pub mod delete_album;
    pub mod get_albums;
    pub mod login;
    pub mod rest;
    pub mod update_album;
}

mod usecases {
    pub mod auth;
    pub mod create_album;
    pub mod create_user;
    pub mod delete_album;
    pub mod errors;
    pub mod get_albums;
    pub mod login;
    pub mod update_album;
}

mod repository {
    pub mod albums;
    pub mod errors;
    pub mod users;
}

mod utils {
    pub mod hash;
    pub mod jwt;
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_conn = config::get_connection_pool();
    let app = Router::new()
        .merge(private_routers())
        .merge(public_routes())
        .with_state(db_conn);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn private_routers() -> Router<DbPool> {
    // https://docs.rs/axum/latest/axum/middleware/index.html#passing-state-from-middleware-to-handlers
    Router::new()
        .route("/get", get(entrypoints::get_albums::handler))
        .route("/create", post(entrypoints::create_album::handler))
        .route("/update", put(entrypoints::update_album::handler))
        .route("/delete/:id", delete(entrypoints::delete_album::handler))
        .layer(tower::ServiceBuilder::new().layer(from_fn(entrypoints::auth::auth)))
}

fn public_routes() -> Router<DbPool> {
    // https://docs.rs/axum/latest/axum/middleware/index.html#passing-state-from-middleware-to-handlers
    Router::new()
        .route("/login", post(entrypoints::login::handler))
        .route("/signup", post(entrypoints::create_user::handler))
}
