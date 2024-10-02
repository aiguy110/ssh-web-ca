use std::error::Error;
use std::str::FromStr;

use dotenv;

mod model;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")?;
    let db_pool = sqlx::SqlitePool::connect(&db_url).await?;
    let ctr = model::controller::Controller::new(db_pool);

    let certificate = ssh_key::Certificate::from_str(
       "ssh-ed25519-cert-v01@openssh.com AAAAIHNzaC1lZDI1NTE5LWNlcnQtdjAxQG9wZW5zc2guY29tAAAAIE7x9ln6uZLLkfXM8iatrnAAuytVHeCznU8VlEgx7TvLAAAAILM+rvN+ot98qgEN796jTiQfZfG1KaT0PtFDJ/XFSqtiAAAAAAAAAAAAAAABAAAAFGVkMjU1MTktd2l0aC1wMjU2LWNhAAAAAAAAAABiUZm7AAAAAPTaMrsAAAAAAAAAggAAABVwZXJtaXQtWDExLWZvcndhcmRpbmcAAAAAAAAAF3Blcm1pdC1hZ2VudC1mb3J3YXJkaW5nAAAAAAAAABZwZXJtaXQtcG9ydC1mb3J3YXJkaW5nAAAAAAAAAApwZXJtaXQtcHR5AAAAAAAAAA5wZXJtaXQtdXNlci1yYwAAAAAAAAAAAAAAaAAAABNlY2RzYS1zaGEyLW5pc3RwMjU2AAAACG5pc3RwMjU2AAAAQQR8H9hzDOU0V76NkkCY7DZIgw+SqoojY6xlb91FIfpjE+UR8YkbTp5ar44ULQatFaZqQlfz8FHYTooOL5G6gHBHAAAAZAAAABNlY2RzYS1zaGEyLW5pc3RwMjU2AAAASQAAACEA/0Cwxhkac5AeNYE958j8GgvmkIESDH1TE7QYIqxsFsIAAAAgTEw8WVjlz8AnvyaKGOUELMpyFFJagtD2JFAIAJvilrc= user@example.com"
    )?;

    let mut cert_model = model::Cert::new(certificate);
    
    cert_model = ctr.upsert_cert(cert_model).await?;
    cert_model = ctr.upsert_cert(cert_model).await?;
    println!("{:?}", cert_model);

    Ok(())
}




