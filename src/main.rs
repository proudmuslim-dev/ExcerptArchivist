use color_eyre::eyre::Result;
use sqlx::SqlitePool;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    Ok(())
}
