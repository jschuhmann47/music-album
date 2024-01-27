pub mod config;
pub mod models;
pub mod schema;

use axum::{http::StatusCode, response::Html, routing::get, Json, Router};
use diesel::{query_dsl::methods::SelectDsl, RunQueryDsl, SelectableHelper};
use models::Album;

#[tokio::main]
async fn main() {
    
    let app = Router::new();
    
    app.route("/", get(handler));
    app.route("/db", get(|| {
        let res = db_handler();
        (StatusCode::CREATED, Json(res))
    }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn db_handler() -> Vec<Album> {
    use self::schema::albums::dsl::*;

    let db = &mut config::connect_to_db();

    albums.select(Album::as_select()).load(db).expect("failed to select results")
}