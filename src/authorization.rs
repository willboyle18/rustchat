use axum_login::{AuthUser, AuthnBackend, UserId};
use sqlx::postgres::PgPool;
use sqlx::Row;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct User {
    id: i64,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &[] // not implemented so empty slice
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

impl Credentials {
    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn get_password(&self) -> &String {
        &self.password
    }
}

impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
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

        match row {
            Ok(Some(row)) => {
                let user = User {
                    id: row.get("id"),
                };
                Ok(Some(user))
            }
            Ok(None) => {
                Ok(None)
            },
            _ => Ok(None),
        }
    }

    async fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {

        let row = sqlx::query(
            r#"
            SELECT id, username, password
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(*user_id)
        .fetch_optional(&self.pool)  // fetch_optional -> Ok(None) if not found
        .await;

        match row{
            Ok(Some(row)) => {
                let user = User {
                    id: row.get("id"),
                };
                Ok(Some(user))
            }
            _ => Ok(None)
        }
    }
}