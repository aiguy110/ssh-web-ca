use axum::extract::FromRef;
use sqlx::{query_as, query, SqlitePool};
use crate::AppState;

use super::*;

#[derive(Debug)]
pub enum Error {
    SqlError(sqlx::Error),
    SerdeJsonError(serde_json::Error)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SqlError(error) => write!(f, "SqlError: {error}"),
            Error::SerdeJsonError(error) => write!(f, "SerdeJsonError: {error}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::SqlError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::SerdeJsonError(value)
    }
}


#[derive(Clone, Debug)]
pub struct ModelController {
    db: SqlitePool
}

impl FromRef<AppState> for ModelController {
    fn from_ref(input: &AppState) -> Self {
        input.model_controller.clone()
    }
}

impl ModelController {
    pub fn new(db: SqlitePool) -> Self {
        ModelController { db }
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, Error> {
        Ok(
            query_as!(User, "SELECT * FROM user WHERE username = ?", username)
                .fetch_optional(&self.db)
                .await?
        )
    }

    pub async fn upsert_cert(&self, cert: Cert) -> Result<Cert, Error> {
        match cert.id {
            None => {
                Ok(
                    query_as!(Cert, "INSERT INTO cert(inner) VALUES (?) RETURNING id, inner as 'inner: Json<Certificate>'", cert.inner)
                        .fetch_one(&self.db)
                        .await?
                )
            },
            Some(id) => {
                let cert_json = serde_json::to_string(&cert.inner)?;
                query!("UPDATE cert SET inner = ? WHERE id = ?", cert_json, id)
                    .execute(&self.db)
                    .await?;

                Ok(cert)
            }
        }
    }
}
