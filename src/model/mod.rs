use sqlx::{
    FromRow,
    types::Json,
};
use ssh_key::certificate::Certificate;

pub mod controller;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
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
