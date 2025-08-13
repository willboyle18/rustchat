use std::{net::SocketAddr, sync::Arc};
use tokio::net::{TcpListener, TcpStream};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use tracing::{info};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    let _ = tracing_subscriber::fmt().init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let app = Router::new().route("/health", get(health));

    let listener = TcpListener::bind(addr).await.unwrap();

    info!("listening on http://{}", addr);

    axum::serve(listener, app)
        .await
        .expect("server error");
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}