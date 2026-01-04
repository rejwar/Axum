use axum::{extract::Form, routing::post, Router};

use sarde::Deserialize;

#[derive(Deserialize, Debug)]

struct Person {
    name: String,
    age: i32,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", post(hello_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("The server is running http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
