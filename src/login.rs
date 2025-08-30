#![allow(warnings)]

use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Form,
    debug_handler
};

use axum::response::Html;
use tokio::fs;
use crate::state::AppState;
use crate::authorization::{Backend, Credentials};
use serde::Deserialize;
use axum::extract::{State, Json};


type AuthSession = axum_login::AuthSession<Backend>;


pub async fn login_get() -> Html<String> {
    let html = fs::read_to_string("WebContent/login.html")
        .await
        .unwrap_or_else(|_| "<h1>Error loading login page</h1>".to_string());

    Html(html)
}

pub async fn login_post(
    State(_state): State<AppState>,
    mut auth_session: AuthSession,
    Json(creds): Json<Credentials>,
) -> impl IntoResponse {
    println!("{:#?}", &creds);
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    } else{
        println!("{:#?}", &user);
    }

    println!("Logged in");

    StatusCode::OK.into_response()
}