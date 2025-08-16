use std::{net::SocketAddr, sync::Arc};
use tokio::net::{TcpListener, TcpStream};
use axum::{
    http::StatusCode,
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{Response, Html, IntoResponse},
    routing::get,
    Router,
};
use axum::extract::ws::Utf8Bytes;
use tracing::{info, warn};
use tracing_subscriber;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let _ = tracing_subscriber::fmt().init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let static_files = ServeDir::new("WebContent")
        .not_found_service(ServeFile::new("WebContent/index.html"));

    let app = Router::new()
        .route("/health", get(health))
        .route("/ws", get(ws_handler))
        .fallback_service(static_files);

    let listener = TcpListener::bind(addr).await.unwrap();

    info!("listening on http://{}", addr);

    axum::serve(listener, app)
        .await
        .expect("server error");
}

async fn health() -> impl IntoResponse {
    info!("/health endpoint called");
    (StatusCode::OK, "ok")
}

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(message) = socket.recv().await {
        let message = if let Ok(message) = message {
            info!("Got message: {:?}", message);
        };
    }
    info!("Client disconnected");
}