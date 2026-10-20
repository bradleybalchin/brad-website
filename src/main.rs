mod features;

use axum::{
    Router, middleware as axum_middleware, routing::{get, post}, serve::Listener
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    
    let app = Router::new()
                .merge(features::home::router())
                .fallback_service(ServeDir::new("public"));


    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
