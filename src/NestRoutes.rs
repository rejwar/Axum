use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    routing::get,
    Router,
};

struct MyExtractor;

#[async_trait]
impl<S> FromRequestParts<S> for MyExtractor
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        if let Some(auth) = parts.headers.get("Authorization") {
            println!(" We got authorization header is {:?}", auth);
            Ok(MyExtractor)
        } else {
            Err((
                StatusCode::UNAUTHORIZED,
                "Didn't get the token ".to_string(),
            ))
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/secret", get(secret_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is Running : http://localhost:3000/secret");
    axum::serve(listener, app).await.unwrap();
}

async fn secret_handler(_auth: MyExtractor) -> String {
    "Congratulations ! U have accessed secret data ".to_string()
}
