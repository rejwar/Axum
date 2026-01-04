use axum::{extract::State, routing::get, Router};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]

struct Person {
    name: String,
    age: u32,
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(Mutex::new(Person {
        name: "Gym".to_string(),
        age: 20,
    }));

    let app = Router::new()
        .route("/hello", get(hello_handler))
        .route("/word", get(word_handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(" Server is Running http: / localhost: 3000");
    axum::serve(listener, app).await.unwrap();
}

async fn hello_handler(State(state): State<Arc<Mutex<Person>>>) -> String {
    let mut person = state.lock().unwrap();
    person.name = "Tom".to_string();
    person.age = 10;

    " Data has been changed ".to_string()
}

async fn word_handler(State(state): State<Arc<Mutex<Person>>>) -> String {
    let person = state.lock().unwrap();
    format!("User is {} , age {}", person.name, person.age)
}
