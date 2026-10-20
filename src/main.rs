mod features;

use axum::{
    Router, middleware as axum_middleware, routing::{get, post}, serve::Listener
};

#[tokio::main]
async fn main() {
    
    let app = Router::new()
                .merge(features::home::router());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
