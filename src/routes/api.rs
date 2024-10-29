use axum_login::AuthSession;
use axum::{extract::State, http::StatusCode, routing::post, Router};
use ssh_key::PublicKey;

use crate::{model::{controller::ModelController, Cert}, ssh_ca::SshCA, AppState};


#[axum::debug_handler]
async fn sign_ssh_public_key(auth_session: AuthSession<ModelController>, State(app_state): State<AppState>, body: String) -> Result<String, StatusCode> {
    let pub_key =  body.parse::<PublicKey>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let user = auth_session.user.ok_or(StatusCode::UNAUTHORIZED)?;

    let mut cert = Cert::new(
        app_state.ssh_ca
            .sign_public_key(&pub_key, &user.username)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    );

    cert = app_state.model_controller
        .upsert_cert(cert).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(cert.to_string())
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/sign_ssh_public_key", post(sign_ssh_public_key))
}
