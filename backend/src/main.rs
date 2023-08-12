#[macro_use]
extern crate rocket;

use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    rocket::build().mount("/", routes![]).launch().await?;

    Ok(())
}
