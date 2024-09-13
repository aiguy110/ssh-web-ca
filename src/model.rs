use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Row,
    sqlite::SqliteRow,
    types::Json,
};
use ssh_key::certificate::Certificate;

#[derive(Debug, FromRow)]
pub struct User {
    id: u64,
    username: String,
}

#[derive(Debug, FromRow)]
pub struct Cert {
    id: Option<u64>,
    inner: Json<Certificate>
}
