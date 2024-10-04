use std::{convert::TryFrom, path::PathBuf};

use serde::Deserialize;
use ssh_key::{PrivateKey, rand_core::OsRng};
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Deserialize)]
pub struct SignatoryConfig {
    private_key_path: PathBuf,
    validity_period_secs: u64
}

#[derive(Clone, Debug)]
pub struct Signatory {
    private_key: PrivateKey,
    validity_period_secs: u64
}

impl TryFrom<SignatoryConfig> for Signatory {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: SignatoryConfig) -> Result<Self, Self::Error> {
        Ok(
            Signatory {
                private_key: PrivateKey::read_openssh_file(&value.private_key_path)
                    .map_err(|e| format!("Error reading private key: {e}"))?,
                validity_period_secs: value.validity_period_secs,
            }
        )
    }
}
