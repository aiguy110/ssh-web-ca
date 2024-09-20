use sqlx::{query_as, Error as SqlError, SqlitePool};
use crate::model::{User, Cert};

pub struct Controller {
    db: SqlitePool
}

impl Controller {
    pub fn new(db: SqlitePool) -> Self {
        Controller { db }
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, SqlError> {
        Ok(
            query_as!(User, "SELECT * FROM user WHERE username = ?", username)
                .fetch_optional(&self.db)
                .await?
        )
    }
}
