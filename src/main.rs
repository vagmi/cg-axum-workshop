mod greeting;

use greeting::greet;
use axum::routing::{get, Router};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(greet));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router.into_make_service()).await.unwrap();
}

