mod constants;
mod server;
mod sol;
mod types;
mod wallet;

use crate::server::Server;
use anyhow::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let port: u16 = args
        .next()
        .unwrap_or_else(|| "11111".to_string())
        .parse()
        .unwrap_or(11111);
    dotenvy::dotenv().ok();
    let rpc_url = env::var("ETH_RPC_URL").unwrap_or_else(|_| "default_key".to_string());
    let private_key = env::var("PRIVATE_KEY").unwrap_or_else(|_| "default_key".to_string());
    let server = Server::new().await?;
    server.run(rpc_url, private_key, port).await?;
    Ok(())
}
