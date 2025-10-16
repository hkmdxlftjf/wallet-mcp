mod wallet;
mod server;

use anyhow::Result;
use std::env;
use crate::server::Server;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let rpc_url = env::var("ETH_RPC_URL").unwrap_or_else(|_| "default_key".to_string());
    let server = Server::new().await?;
    server.run(rpc_url).await?;
    Ok(())
}