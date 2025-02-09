mod config;
mod tracer;
mod utils;

use clap::Parser;
use config::Config;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::parse();
    tracer::trace_transaction(&config.rpc_url, config.hash, config.search).await?;
    Ok(())
}