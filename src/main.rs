use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // Build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello from Rust running via ArgoCD! v1.0" }));

    // Run it on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
