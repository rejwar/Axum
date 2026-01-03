use axum::{extract::Query, routing::get, Router};

use serde::Deserialize;
#[derive(Deserialize)]
struct HelloQuery {
    id: Option<u32>,
    name: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(hello_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello_handler(Query(params): Query<HelloQuery>) -> String {
    format!(" ID {:?}, name {}", params.id, params.name)
}
