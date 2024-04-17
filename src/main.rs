use axum::{routing::get, Router};

fn hello_world() -> String {
    "Hello, world!".to_string()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world()));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
