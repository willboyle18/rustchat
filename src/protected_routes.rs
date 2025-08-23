use axum::{routing::get, Router};
use axum_login::login_required;

fn protected_routes() -> Router {
    Router::new()
        .route(
            "/protected",
            get(|| async { "Gotta be logged in to see me!" }),
        )
        .route_layer(login_required!(Backend, login_url = "/login"))
}