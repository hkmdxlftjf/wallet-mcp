mod constants;
mod sol;
mod types;
mod server;
use anyhow::Result;
use std::env;
use rmcp::ServiceExt;
use rmcp::transport::stdio;
use tracing_subscriber::{util::SubscriberInitExt, EnvFilter, {self}};
use crate::server::McpServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the tracing subscriber with file and stdout logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting MCP server");

    // dotenvy::dotenv()?;
    let rpc_url = env::var("ETH_RPC_URL").unwrap_or_else(|_| "default_key".to_string());
    let private_key = env::var("PRIVATE_KEY").unwrap_or_else(|_| "default_key".to_string());
    // Create an instance of our counter router
    let service = McpServer::new(private_key, rpc_url).serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;
    Ok(())
}

