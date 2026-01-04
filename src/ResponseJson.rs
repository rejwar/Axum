use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct UserTask {
    id: u32,
    task_name: String,
    is_completed: bool,
}

#[tokio::main]

async fn main() {
    let app = Router::new().route("/process_task", post(task_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server is running http: //localhost:3000/process_task");
    axum::serve(listener, app).await.unwrap();
}

async fn task_handler(Json(mut input): Json<UserTask>) -> Json<UserTask> {
    input.task_name = format!(" The process has been done {}", input.task_name);
    input.is_completed = true;

    Json(input)
}
