use axum::{async_trait, extract::FromRef};
use axum_login::{AuthnBackend, UserId};
use samael::schema::Assertion;
use sqlx::{query_as, query, SqlitePool};
use crate::AppState;

use super::*;

#[derive(Debug)]
pub enum Error {
    SqlError(sqlx::Error),
    SerdeJsonError(serde_json::Error),
    FailedToRetrieveUsernameFromAssertion { assertion: Assertion },
    UserIdMustBeNoneOnCreate,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SqlError(error) => write!(f, "SqlError: {error}"),
            Error::SerdeJsonError(error) => write!(f, "SerdeJsonError: {error}"),
            Error::FailedToRetrieveUsernameFromAssertion { assertion } => write!(f, "Failed to retrieve username from assertion: {assertion:?}"),
            Error::UserIdMustBeNoneOnCreate => write!(f, "User.id must be None at user creation.")
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

#[async_trait]
impl AuthnBackend for ModelController {
    #[doc = " Authenticating user type."]
    type User = User;

    #[doc = " credential type used for authentication."]
    type Credentials = samael::schema::Assertion;

    #[doc = " An error which can occur during authentication and authorization."]
    type Error = Error;

    #[doc = " Authenticates the given credentials with the backend."]
    async fn authenticate(&self, creds: Self::Credentials) -> Result<Option<Self::User>, Self::Error> {
        // We don't validate the Assertion here because we will only ever get an assertion from
        // ServiceProvider::parse_base64_assertion, which performs validation during parsing.
        let username = creds.clone().subject
            .ok_or_else(|| { Error::FailedToRetrieveUsernameFromAssertion {assertion: creds.clone()}})?
            .name_id
            .ok_or_else(|| { Error::FailedToRetrieveUsernameFromAssertion {assertion: creds}})?
            .value;

        let user = self.get_or_create_user_by_username(&username).await?;
        Ok(Some(user))
    }

    #[doc = " Gets the user by provided ID from the backend."]
    async fn get_user<>(&self, user_id: &UserId<Self>) ->  Result<Option<Self::User> ,Self::Error> {
        self.get_user_by_id(*user_id).await
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

    pub async fn get_or_create_user_by_username(&self, username: &str) -> Result<User, Error> {
        let fetched_user = self.get_user_by_username(username).await?;
        match fetched_user {
            Some(user) => Ok(user),
            None => Ok(
                query_as!(User, "INSERT INTO user (username) VALUES (?) RETURNING id, username", username)
                    .fetch_one(&self.db)
                    .await?
            )
        }
    }

    pub async fn get_user_by_id(&self, user_id: i64) -> Result<Option<User>, Error> {
        Ok(
            query_as!(User, "SELECT * FROM user WHERE id = ?", user_id)
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
