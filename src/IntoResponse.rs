use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/success", get(success_handler))
        .route("/error", get(error_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(" Server is Running http://localhost: 3000");
    axum::serve(listener, app).await.unwrap();
}

async fn success_handler() -> impl IntoResponse {
    (StatusCode::OK, "Everything is okay")
}

async fn error_handler() -> impl IntoResponse {
    (StatusCode::BAD_REQUEST, "U have sent wrong Request")
}
