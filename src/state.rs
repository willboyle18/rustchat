use tokio::sync::broadcast;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub tx: broadcast::Sender<String>,
    pub pool: PgPool,
}

impl AppState {
    pub fn new(tx: broadcast::Sender<String>, pool: PgPool) -> Self {
        Self { tx, pool }
    }
}