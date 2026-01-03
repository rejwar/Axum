use axum::{extract::Path, routing::get, Router};

#[tokio::main]

async fn main() {
    let app = Router::new()
        .route("/profile/:username", get(profile_handler))
        .route("/user/:user_id/post_id", get(post_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is runnig");
    axum::serve(listener, app).await.unwrap();
}

async fn profile_handler(Path(username): Path<String>) -> String {
    format!("{} welcome to my profile ", username)
}

async fn post_handler(Path(user_id, post_id)):Path<(u32, u32)>-> String {
    format!(" User no. {} , Post no {}", user_id, post_id)
}
