use tower_http::services::{ServeDir, ServeFile};
use tokio;
use axum::response::Html;

pub async fn login() -> Html<&'static str> {
    println!("Hello from login");
    Html(include_str!("../WebContent/login.html"))
}