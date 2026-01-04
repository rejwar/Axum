use axum::{
    body::Body,
    http::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Home page" }))
        .route("/hello", get(|| async { "Hello Page" }))
        .route("/word", get(|| async { "Word Page" }))
        .route_layer(middleware::from_fn(my_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server is Running: http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn my_middleware(req: Request<Body>, next: Next) -> Response {
    println!("Middleware: Request is being checked");

    let res = next.run(req).await;

    println!("Middleware: Response is being checked before sending");

    res
}
