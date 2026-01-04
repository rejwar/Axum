use axum::{
    http::{self, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};

#[tokio::main]

async fn main() {
    let app = Router::new()
        .route("/success", get(success_handled))
        .route("/created", get(created_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is Running  http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn success_handled() -> impl IntoResponse {
    (StatusCode::OK, "Operation successfull")
}

async fn created_handler() -> impl IntoResponse {
    (
        StatusCode::CREATED,
        [("X-custom-header", " my-value")],
        "New Resources built",
    )
}
