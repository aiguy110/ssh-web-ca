use axum::extract::FromRef;
use samael::{metadata::EntityDescriptor, service_provider::{ServiceProvider, ServiceProviderBuilder}};
use serde::Deserialize;
use std::fs;

use crate::AppState;

#[derive(Deserialize)]
pub struct SamlSPConfig {
    idp_metadata_url: String,
    idp_login_url: Option<String>,
    sp_entity_id: String,
    cert_path: String,
    private_key_path: String,
    acs_url: String,
}

impl SamlSPConfig {
    pub async fn try_instantiate(self) -> Result<SamlSPState, Box<dyn std::error::Error>> {
        // Try the thing most likely to fail first
        let resp = reqwest::get(self.idp_metadata_url).await?;
        let idp_metadata_text = resp.text().await?;
        let idp_metadata: EntityDescriptor = samael::metadata::de::from_str(&idp_metadata_text)?;

        let cert = openssl::x509::X509::from_pem(&fs::read(self.cert_path)?)?;
        let private_key = openssl::rsa::Rsa::private_key_from_pem(&fs::read(self.private_key_path)?)?;

        let inner_sp = ServiceProviderBuilder::default()
            .entity_id(self.sp_entity_id)
            .key(private_key)
            .certificate(cert)
            .allow_idp_initiated(true)
            .idp_metadata(idp_metadata)
            .acs_url(self.acs_url)
            .build()?;

        Ok(SamlSPState{
            inner_sp,
            idp_login_url: self.idp_login_url
        })
    }
}

#[derive(Clone)]
pub struct SamlSPState {
    pub inner_sp: ServiceProvider,
    pub idp_login_url: Option<String>
}

impl std::fmt::Debug for SamlSPState {
    // Have to derive this manually because `samael::service_provider::ServiceProvider` doesn't
    // impliment `Debug`... probably to protect the private key it holds.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServiceProviderState {{...}}")
    }
}

impl FromRef<AppState> for SamlSPState {
    fn from_ref(input: &AppState) -> Self {
        input.saml_sp.clone()
    }
}


