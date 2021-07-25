mod lib;
use lib::run;
use std::error::Error;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn Error>> {
    run::from_cli().await?;

    Ok(())
}
