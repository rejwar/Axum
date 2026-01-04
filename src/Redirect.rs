use axum::{
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(hello_handler))
        .route("/world", get(world_redirect_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is running : http://localhost:3000/world");
    axum::serve(listener, app).await.unwrap();
}

async fn hello_handler() -> String {
    "Hello U have come to redirect".to_string()
}

async fn world_redirect_handler() -> impl IntoResponse {
    Redirect::to("/hello")
}
