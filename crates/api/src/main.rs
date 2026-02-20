//! TravelTrust API 入口：Axum + CORS，路由预留

use axum::{
    routing::get,
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/guides", get(guides_list_placeholder))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("TravelTrust API listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    axum::serve(listener, app).await.expect("serve");
}

async fn health() -> &'static str {
    "ok"
}

async fn guides_list_placeholder() -> &'static str {
    "[]"
}
