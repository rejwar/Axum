use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct UserResponse {
    id: u32,
    username: String,
    status: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/user/profile", get(get_user_profile));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is Running : http: //localhost:3000/user/profile");
    axum::serve(listener, app).await.unwrap();
}

async fn get_user_profile() -> Json<UserResponse> {
    let response = UserResponse {
        id: 1,
        username: "Rakib Rustacean".to_string(),
        status: "active".to_string(),
    };

    Json(response)
}
