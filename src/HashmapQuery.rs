use axum::{extract::Query, routing::get, Router};

use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(hello_handler));

    let listener = tokio::net::TcpListener::bind(("0.0.0.0:3000"))
        .await
        .unwrap();
    println!("Server is Running : Http://localhost:3000/hello?name=gym&age=20");
    axum::serve(listener, app).await.unwrap();
}

async fn hello_handler(Query(params): Query<HashMap<String, String>>) -> String {
    for (key, value) in &params {
        println!(" Key {} , value {}", key, value);
    }
    format!("we got {} parameter in total ", params.len())
}
