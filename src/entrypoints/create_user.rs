use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::{json, Value};

use super::rest;
use crate::{config, usecases};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

pub async fn handler(
    State(db_conn): State<config::DbPool>,
    Json(req): Json<CreateUserRequest>,
) -> (StatusCode, Json<Value>) {
    let res = usecases::create_user::execute(db_conn, req);

    match res {
        Ok(_) => (StatusCode::OK, Json(json!({}))),
        Err(e) => match e {
            usecases::errors::UsecaseError::DatabaseError(desc) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                rest::descf("Database error: ".to_owned().push_str(desc.as_str())),
            ),
            other => (StatusCode::BAD_REQUEST, rest::descf(other)),
        },
    }
}
