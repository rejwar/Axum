use axum::{routing::get, Router};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let static_files_services = ServeDir::new("./assets");

    let app = Router::new()
        .route("/", get(|| async { "Home Page" }))
        .nest_service("/static", static_files_services);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(" Server is Running : http://localhost:3000/static/index.html");
    axum::serve(listener, app).await.unwrap();
}
