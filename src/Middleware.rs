use axum::{extract::Extension, routing::get, Router};

use std::sync::Arc;

struct ApiConfig {
    version: String,
}

#[tokio::main]
async fn main() {
    let shared_config = Arc::new(ApiConfig {
        version: "1.0.0".to_string(),
    });

    let app = Router::new()
        .route("/version", get(show_version))
        .layer(Extension(shared_config));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(" Server is Running http:// localhost: 3000 / version");
    axum::serve(listener, app).await.unwrap();
}

async fn show_version(Extension(confing): Extension<Arc<ApiConfig>>) -> String {
    format!(" Api version {}", confing.version)
}
