use std::{convert::TryFrom, path::PathBuf};

use serde::Deserialize;
use ssh_key::{certificate::{Builder, CertType}, rand_core::{OsRng, RngCore}, Certificate, Error, PrivateKey, PublicKey};
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Deserialize, Clone)]
pub struct SshCAConfig {
    private_key_path: PathBuf,
    valid_principals: Option<Vec<String>>,
    validity_period_secs: u64
}

#[derive(Clone, Debug)]
pub struct SshCA {
    private_key: PrivateKey,
    valid_principals: Option<Vec<String>>,
    validity_period_secs: u64
}

impl TryFrom<SshCAConfig> for SshCA {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: SshCAConfig) -> Result<Self, Self::Error> {
        Ok(
            SshCA {
                private_key: PrivateKey::read_openssh_file(&value.private_key_path)
                    .map_err(|e| format!("Error reading private key: {e}"))?,
                valid_principals: value.valid_principals,
                validity_period_secs: value.validity_period_secs,
            }
        )
    }
}

impl SshCA {
    pub fn sign_public_key(&self, pub_key: &PublicKey, username: &str) -> Result<Certificate, Error>{
        let valid_after = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let valid_before = valid_after + self.validity_period_secs; // e.g. 1 year

        let mut cert_builder = Builder::new_with_random_nonce(
            &mut OsRng,
            pub_key,
            valid_after,
            valid_before,
        )?;

        let ser_num = OsRng.next_u64();
        cert_builder.serial(ser_num);
        cert_builder.key_id(format!("{username}-{ser_num}"));
        cert_builder.comment(username);
        cert_builder.cert_type(CertType::User);
         
        match &self.valid_principals {
            None => { 
                cert_builder.all_principals_valid();}
            ,
            Some(principals) => {
                for p in principals {
                    cert_builder.valid_principal(p);
                }
            }
        } 
        cert_builder.sign(&self.private_key)
    } 
}
