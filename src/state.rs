use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct AppState {
    pub tx: broadcast::Sender<String>,
}

impl AppState {
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(1024);
        Self { tx }
    }
}