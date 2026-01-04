use axum::{routing::post, Json, Router};
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    username: String,
    id: u32,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/user", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_user(Json(payload): Json<User>) -> String {
    format!(" User {} (Id {} is made ", payload.username, payload.id)
}
