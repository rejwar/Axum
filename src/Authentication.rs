use axum::{
    extract::{Json, State},
    http::StatusCode,
    reponse::IntoResponse,
    routing::post,
    Router,
};

use bcrypt::verify;
use serde_json::Json;

async fn sign_in(
    State(client): State<sqlx::PgPool>,
    Json(user_req): Json<UserRequest>,
) -> impl IntoResponse {
    let row = sqlx::query("SELECT password FROM users WHERE username = $1")
        .bind(&user_req.username)
        .fetch_one(&client)
        .await;

    let hash_password = match row {
        Ok(r) => r.get::<String, _>(0),
        Err(_) => return (StatusCode::UNAUTHORIZED, "Incorrect password").into_response(),
    };

    let is_valid = verify(&user_req.password, &hash_password).unwrap_or(false);

    if is_valid {
        (StatusCode::OK, "Sign in successful").into_response()
    } else {
        (StatusCode::UNAUTHORIZED, "Incorrect password").into_response()
    }
}

pub fn app() -> Router {
    Router::new().route("/user/signin", post(sign_in))
}
