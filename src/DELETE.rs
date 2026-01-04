use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/hello",
        get(hello_handler)
            .post(post_handler)
            .put(put_handler)
            .delete(delete_handler),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(" Server is runnig : http://localhost:3000/hello");
    axum::serve(listener, app).await.unwrap();
}

async fn hello_handler() -> &'static str {
    "GET request (hello) [00:02:10]"
}

async fn post_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        "This is post request (post hello) [00:02:10]",
    )
}

async fn put_handler() -> impl IntoResponse {
    (StatusCode::OK, "This PUT request (put hello) [00:02:10]")
}

async fn delete_handler() -> impl IntoResponse {
    (StatusCode::OK, "DELETE request (Delete hello) [00:02:27]")
}
