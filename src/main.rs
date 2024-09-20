use std::error::Error;

use dotenv;

mod model;
mod controller;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")?;
    let db_pool = sqlx::SqlitePool::connect(&db_url).await?;

    let ctr = controller::Controller::new(db_pool);

    let user = ctr.get_user_by_username("josiah.hunsinger")
        .await?;

    println!("{:?}", user);

    Ok(())
}


