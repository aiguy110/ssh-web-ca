use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Row,
    sqlite::SqliteRow,
    types::Json,
};
use ssh_key::certificate::Certificate;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(Debug, FromRow)]
pub struct Cert {
    id: Option<u64>,
    inner: Json<Certificate>
}
