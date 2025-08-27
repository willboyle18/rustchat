#![allow(warnings)]

use axum_login::{AuthUser, AuthnBackend, UserId};
use sqlx::postgres::{PgPoolOptions, PgRow, PgPool};
use sqlx::{FromRow, Row};
use serde::Deserialize;
use url::quirks::{password, username};

#[derive(Debug, Clone)]
pub struct User {
    id: i64,
    username: String,
    password: String,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        b"testpw"
    }
}

#[derive(Clone)]
pub struct Backend {
    pub pool: PgPool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Credentials {
    username: String,
    password: String,
}

impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        println!("hello there");

        println!("username: {}", &creds.username);
        println!("password: {}", &creds.password);
        let row = sqlx::query(
            r#"
            SELECT id, username, password
            FROM users
            WHERE username = $1 AND password = $2
            "#,
        )
        .bind(&creds.username)
        .bind(&creds.password)
        .fetch_optional(&self.pool)  // fetch_optional -> Ok(None) if not found
        .await;

        println!("did we get here?");

        match row {
            Ok(Some(row)) => {
                println!("hello there");
                let user = User {
                    id: row.get("id"),
                    username: row.get("username"),
                    password: row.get("password"),
                };
                println!("hello again");
                Ok(Some(user))
            }
            Ok(None) => Ok(None),
            _ => Ok(None),
        }
    }

    async fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        // HARDCODED FOR DEBUGGING PURPOSES
        let username = String::from("test");
        let password = String::from("testpw");

        Ok(Some(User {
            id: 0,
            username: username,
            password: password,
        }))
    }
}