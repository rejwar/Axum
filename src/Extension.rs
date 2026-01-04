use axum::{
    extract::{Extension, Request},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use std::sync::Arc;

// ১. ডাটা স্ট্রাকচার (Person)
#[derive(Debug, Clone)]
struct Person {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        // '/word' রাউটে মিডলওয়্যার এবং হ্যান্ডলার যুক্ত করা হয়েছে
        .route("/word", get(word_handler))
        .route_layer(middleware::from_fn(hello_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("সার্ভার চলছে: http://localhost:3000/word");
    axum::serve(listener, app).await.unwrap();
}

// ২. মিডলওয়্যার ফাংশন: এখানে রিকোয়েস্টে ডাটা ইনসার্ট করা হয়
async fn hello_middleware(mut request: Request, next: Next) -> Response {
    let person = Arc::new(Person {
        id: 0,
        name: "Gym".to_string(),
    });

    // রিকোয়েস্টের সাথে এক্সটেনশন হিসেবে ডাটা যুক্ত করা
    request.extensions_mut().insert(person);

    next.run(request).await
}

// ৩. হ্যান্ডলার ফাংশন: এখানে এক্সটেনশন থেকে ডাটা পড়া হয়
async fn word_handler(Extension(person): Extension<Arc<Person>>) -> String {
    println!("হ্যান্ডলারে পাওয়া ডাটা: {:?}", person);
    format!("হ্যালো {}, আপনার আইডি: {}", person.name, person.id)
}
