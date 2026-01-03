use axum::{extract::Path, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/word/:id", get(word_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is Running http:// localhost : 3000/word/5");

    axum::serve(listener, app).await.unwrap();
}
async fn word_handler(Path(id): Path<u32>) -> String {
    format!(" The ID u have sent {}", id)
}
