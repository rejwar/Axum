use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/hello",
            get(|| async { " GET Request : data is reading " }),
        )
        .route(
            "/hell",
            post(|| async { " POST new request : new data is building" }),
        )
        .route(
            "/hello",
            put(|| async { "PUT request : Data is updating fully" }),
        )
        .route(
            "/hello",
            patch(|| async { "PATCH request: Data is updating partly" }),
        )
        .route(
            "/hello",
            delete(|| async { "DELETE Request : Data is deleted " }),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is runnig : http://localhost:3000/hello");

    axum::serve(listener, app).await.unwrap();
}
