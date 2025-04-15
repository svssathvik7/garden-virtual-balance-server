use axum::{response::IntoResponse, routing::get, Router};
use tokio::net::TcpListener;

async fn greet() -> impl IntoResponse {
    "Welcome to garden virtual balances"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(greet));
    let addr = "0.0.0.0:3001";
    let tcp_listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", &addr);
    axum::serve(tcp_listener, app).await.unwrap();
}
