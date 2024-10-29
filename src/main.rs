use std::error::Error;

use axum::{extract::State, routing::{get, post}, Router};
use axum_login::{login_required, tower_sessions::{MemoryStore, SessionManagerLayer}, AuthManagerLayerBuilder};
use dotenv;
use model::controller::ModelController;
use serde::Deserialize;
use ssh_ca::{SshCA, SshCAConfig};
use service_provider::{SamlSPConfig, SamlSPState};
use tracing::info;

mod model;
mod routes;
mod ssh_ca;
mod service_provider;

#[derive(Deserialize)]
pub struct AppConfig {
    saml_sp_config: SamlSPConfig,
    ssh_ca_config: SshCAConfig,
    listen_sock_addr: String
}

#[derive(Debug, Clone)]
pub struct AppState {
    model_controller: ModelController,
    saml_sp: SamlSPState,
    ssh_ca: SshCA
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load .env file, then init logging
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    // Load config
    let config_path = std::env::var("SSH_WEB_CA_CONFIG")
        .unwrap_or("./config.yml".to_owned());
    let config_str = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file: {e}"))?;
    let app_config: AppConfig = serde_yaml::from_str(&config_str)?;

    // Init AppState and sub states
    let app_state = AppState {
        model_controller: init_model_controller().await?,
        saml_sp: app_config.saml_sp_config.try_instantiate().await?,
        ssh_ca: app_config.ssh_ca_config.try_into()?
    };

    // Init session_layer
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    // Init auth_layer
    let auth_layer = AuthManagerLayerBuilder::new(app_state.model_controller.clone(), session_layer).build();


    let app = Router::new()
        .route("/", get(landing_page))
        .nest("/api", routes::api::routes())
        .route_layer(login_required!(ModelController, login_url = "/login"))
        .merge(routes::auth::routes())
        .with_state(app_state)
        .layer(auth_layer);

    let listener = tokio::net::TcpListener::bind(&app_config.listen_sock_addr).await?;

    info!("Listening at {}", app_config.listen_sock_addr);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn init_model_controller() -> Result<ModelController, Box<dyn Error>> {
    let db_url = std::env::var("DATABASE_URL").unwrap_or("ssh-ca.db".to_owned());
    let db_pool = sqlx::SqlitePool::connect(&db_url).await?;
    Ok(ModelController::new(db_pool))
}

async fn landing_page() -> &'static str {
    "Nothing here yet. Try visiting again some time."
}

