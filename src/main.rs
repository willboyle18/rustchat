#![allow(warnings)]

mod state;
mod messages;
mod login;
mod authorization;

use std::{net::SocketAddr, sync, sync::Arc};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use axum::{
    http::StatusCode,
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade, Utf8Bytes},
        State,
    },
    response::{Response, Html, IntoResponse, Redirect},
    routing::{get, get_service, post},
    Router,
    debug_handler
};
use axum_login::{login_required, tower_sessions::{MemoryStore, SessionManagerLayer}, AuthManagerLayerBuilder, AuthSession};
use tracing::{info, warn};
use tracing_subscriber;
use tower_http::services::{ServeDir, ServeFile};
use futures::{StreamExt, SinkExt};
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};

use crate::state::AppState;
use crate::messages::{ServerMessage, ChatMessage};
use crate::authorization::Backend;

#[tokio::main]
async fn main() {
    let _ = tracing_subscriber::fmt().init();

    let (tx, _rx) = broadcast::channel(1024);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://will:abc123@localhost:5432/rustchat_db?sslmode=disable")
        .await
        .unwrap();

    let state = AppState::new(tx, pool.clone());

    // Session layer.
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false);

    // Auth service.
    let backend = Backend{pool: pool.clone()};
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let static_files = ServeDir::new("WebContent")
        .not_found_service(ServeFile::new("WebContent/login.html"));

    let app = Router::new()
        .route("/", get(|| async { Redirect::to("/login_get") }))
        .route("/ws", get(ws_handler))
        .route_layer(login_required!(Backend, login_url = "/login_get"))
        .route("/login_get", get(login::login_get))
        .route("/login", post(login::login_post))
        .route("/health", get(health))
        .layer(auth_layer)
        .fallback_service(static_files)
        .with_state(state.clone());

    let listener = TcpListener::bind(addr).await.unwrap();

    info!("listening on http://{}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .expect("server error");
}

async fn health() -> impl IntoResponse {
    info!("/health endpoint called");
    (StatusCode::OK, "ok")
}

async fn redirect() -> Redirect {
    Redirect::to("/login")
}

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>, auth: AuthSession<Backend>) -> Response {
    let user_id_from_session = auth.session.get::<i64>("user_id").await.ok().flatten().unwrap();

    ws.on_upgrade(move |socket| {handle_socket(socket, state, user_id_from_session) })
}

async fn handle_socket(socket: WebSocket, state: AppState, user_id_from_session: i64) {
    let mut rx = state.tx.subscribe();
    let (mut sender, mut receiver) = socket.split();

    let username_query = sqlx::query!(
        r#"
        SELECT username
        FROM users
        WHERE id = $1
        "#,
        user_id_from_session
    )
    .fetch_one(&state.pool)
    .await;

    let username: String = username_query.unwrap().username;

    println!("username: {}", username);

    let _ = state.tx.send(serde_json::to_string(
        &ServerMessage::System{ message: "A user joined".into() }
    ).unwrap());

    if let Ok(rows) = sqlx::query!(
        r#"
        SELECT text
        FROM messages
        ORDER BY message_id ASC
        "#
    ).fetch_all(&state.pool).await {
        for row in rows {
            let out = ServerMessage::Chat { user: "anon".into(), text: row.text };
            let json = serde_json::to_string(&out).unwrap();
            if sender.send(Message::Text(json.into())).await.is_err() {
                return;
            }
        }
    }

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
                if let Ok(ChatMessage::Chat { user, text }) = serde_json::from_str(&message) {
                    sqlx::query(
                        r#"
                        INSERT INTO messages(text)
                        VALUES ($1)
                        "#,
                    )
                    .bind(&text)
                    .execute(&state.pool).await.unwrap();
                    let out = ServerMessage::Chat { user, text };
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