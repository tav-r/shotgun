mod lib;
use lib::run;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run::from_cli().await?;

    Ok(())
}
