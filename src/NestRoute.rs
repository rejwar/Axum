use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

#[tokio::main]
async fn main() {
    let user_router = Router::new()
        .route("/profile", get(profile_handler))
        .route("/settings", get(settings_handler));

    let app = Router::new()
        .route("/", get(|| async { "Home page" }))
        .nest("/user", user_router);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is running : http://localhost:3000/user/profiel");
    axum::serve(listener, app).await.unwrap();
}

async fn profile_handler() -> impl IntoResponse {
    (StatusCode::OK, "This is user profile page [00:02:26]")
}

async fn settings_handler() -> impl IntoResponse {
    (StatusCode::OK, " This is user setting page [00:02:37]")
}
