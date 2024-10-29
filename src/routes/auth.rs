use std::collections::HashMap;

use axum::{extract::State, http::StatusCode, response::{IntoResponse, Redirect}, routing::{get, post}, Form, Router};
use axum_login::{AuthSession, AuthnBackend};
use tracing::{error, info};

use crate::{model::controller::ModelController, service_provider::SamlSPState, AppState};

#[axum::debug_handler]
pub async fn login(mut auth_session: AuthSession<ModelController>, State(sp_state): State<SamlSPState>, Form(form_data): Form<HashMap<String, String>>) -> Result<(), StatusCode> {
    if let Some(base64_assertion) = form_data.get("SAMLResponse") {
        let assertion = sp_state.inner_sp.parse_base64_response(base64_assertion, None)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let user = auth_session.backend
            .authenticate(assertion)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?;

        auth_session.login(&user)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        info!("User {} successfully authenticated", user.username);

        Ok(())
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

#[axum::debug_handler]
pub async fn login_idp_redirect(State(sp_state): State<SamlSPState>) -> impl IntoResponse {
    if let Some(redirect_url) = sp_state.idp_login_url {
        Redirect::to(&redirect_url).into_response()
    } else {
        "Please sign in from your IdP".into_response()
    }
}

#[axum::debug_handler]
pub async fn logout(mut auth_session: AuthSession<ModelController>) -> Result<(), StatusCode> {
    match auth_session.user.clone() {
        None => {
            error!("Unknown user attempted to logout");
            Err(StatusCode::UNAUTHORIZED)
        },
        Some(u) => {
            info!("User {} is logging out", u.username);
            auth_session.logout()
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(())
        }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login).get(login_idp_redirect))
        .route("/logout", get(logout))
}
