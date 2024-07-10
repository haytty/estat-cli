use estat_cli::cli;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = cli::start().await?;
    println!("{}", "finish!");
    Ok(())
}
