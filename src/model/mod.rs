use axum_login::AuthUser;
use sqlx::{
    FromRow,
    types::Json,
};
use ssh_key::certificate::Certificate;

pub mod controller;

#[derive(Clone, Debug, FromRow)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id.unwrap_or(0) // TODO: Make dedicated `CreateUser` type to avoid this hack
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.username.as_bytes()
    }
}

#[derive(Debug, FromRow)]
pub struct Cert {
    id: Option<i64>,
    inner: Json<Certificate>
}

impl Cert {
    pub fn new(inner: Certificate) -> Self {
        Cert {
            id: None,
            inner: Json(inner)
        }
    }
}

impl ToString for Cert {
    fn to_string(&self) -> String {
        self.inner.to_string()
    }
}
