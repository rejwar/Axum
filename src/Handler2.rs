use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]

async fn main() {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/user", get(get_user_info))
        .route("/login", post(login_attempt));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "Ok"
}

async fn get_user_info() -> String {
    format!("User id {} , name {}", 101, "Rakib")
}

async fn login_attempt() -> &'static str {
    "log in successfull"
}
