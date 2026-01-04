//
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]

struct User {
    name: String,
}

async fn create_user_handle() -> impl IntoResponse {
    let user = User {
        name: "Rakin".to_string(),
    };

    (StatusCode::CREATED, Json(user))
}
