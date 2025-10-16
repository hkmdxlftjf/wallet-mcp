mod server;
mod wallet;
mod constants;
mod types;
mod sol;

use crate::server::Server;
use anyhow::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let rpc_url = env::var("ETH_RPC_URL").unwrap_or_else(|_| "default_key".to_string());
    let private_key = env::var("PRIVATE_KEY").unwrap_or_else(|_| "default_key".to_string());
    let server = Server::new().await?;
    server.run(rpc_url, private_key).await?;
    Ok(())
}
