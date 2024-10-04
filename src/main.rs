use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;

use axum::{extract::State, routing::get, Router};
use dotenv;
use model::controller::ModelController;
use signatory::{Signatory, SignatoryConfig};

mod auth;
mod model;
mod routes;
mod signatory;

#[derive(Debug, Clone)]
pub struct AppState {
    model_controller: ModelController,
    signatory: Signatory
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let app_state = AppState {
        model_controller: init_model_controller().await?,
        signatory: init_signatory()?
    };

    let app = Router::new()
        .route("/", get(give_state))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[axum::debug_handler]
async fn give_state(State(app_state): State<AppState>) -> String {
    format!("{:#?}", app_state) 
}

async fn init_model_controller() -> Result<ModelController, Box<dyn Error>> {
    let db_url = std::env::var("DATABASE_URL").unwrap_or("ssh-ca.db".to_owned());
    let db_pool = sqlx::SqlitePool::connect(&db_url).await?;
    Ok(ModelController::new(db_pool))
}

fn init_signatory() -> Result<Signatory, Box<dyn Error>> {
    let config_path = std::env::var("SSH_WEB_CA_CONFIG")
        .unwrap_or("./config.yml".to_owned());

    let config_str = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file: {e}"))?;

    let sig_conf: SignatoryConfig = serde_yaml::from_str(&config_str)
        .map_err(|e| format!("Error parsing config file: {e}"))?;

    Ok(sig_conf.try_into()?)
}




