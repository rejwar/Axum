use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

#[tokio::main]

async fn main() {
    let app = Router::new()
        .route("/", get(|| async { " Home page" }))
        .route("/hello", get(|| async { "Hello" }))
        .fallback(not_founcd_handler);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is running : http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn not_founcd_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "SOrry I didn't found the page ()404")
}
