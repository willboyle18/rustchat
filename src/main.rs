mod state;
mod messages;

use std::{net::SocketAddr, sync, sync::Arc};
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
use futures::{StreamExt, SinkExt};
use crate::state::AppState;

#[tokio::main]
async fn main() {
    let _ = tracing_subscriber::fmt().init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let static_files = ServeDir::new("WebContent")
        .not_found_service(ServeFile::new("WebContent/index.html"));

    let state = state::AppState::new();

    let app = Router::new()
        .route("/health", get(health))
        .route("/ws", get(ws_handler))
        .fallback_service(static_files)
        .with_state(state.clone());

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

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(move |socket| {handle_socket(socket, state) })
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let mut rx = state.tx.subscribe();

    let _ = state.tx.send(serde_json::to_string(
        &messages::ServerMessage::System{ message: "A user joined".into() }
    ).unwrap());

    let (mut sender, mut receiver) = socket.split();
    let writer_task = tokio::spawn( async move {
        while let Ok(message) = rx.recv().await {
            info!("sending to client: {message}");
            if sender.send(Message::Text(message.into())).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(message)) = receiver.next().await {
        match message {
            Message::Text(message) => {
                if let Ok(messages::ChatMessage::Chat { user, text }) = serde_json::from_str(&message) {
                    let out = messages::ServerMessage::Chat { user, text };
                    let _ = state.tx.send(serde_json::to_string(&out).unwrap());
                }
                println!("{}", message);
            }
            Message::Close(frame) => {
                info!("client closed: {:?}", frame);
                break;
            }
            _ => {
                println!("Unhandled message: {:?}", message);
            }
        }
    }

    info!("Client disconnected");
}