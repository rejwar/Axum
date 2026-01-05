use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
async fn protected_handler(Extension(username): Extension<String>) -> impl IntoResponse {
    let message = format!("hello {}", username);
    (StatusCode::OK, message)
}
