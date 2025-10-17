use crate::models::IERC20;
use crate::models::{GetBalanceRequest, GetBalanceResponse, MetaData};
use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{model::*, ErrorData as McpError};
use std::str::FromStr;

#[derive(Clone)]
pub struct BalanceHandler {
    rpc_url: String,
}

impl BalanceHandler {
    pub fn new(rpc_url: String) -> Self {
        Self { rpc_url }
    }

    pub async fn handle_get_balance(
        &self,
        Parameters(GetBalanceRequest {
            wallet_address,
            token_address,
        }): Parameters<GetBalanceRequest>,
    ) -> Result<CallToolResult, McpError> {
        let address = Address::from_str(wallet_address.as_str()).map_err(|e| {
            McpError::invalid_params(format!("Invalid wallet address: {}", e), None)
        })?;
        let provider = ProviderBuilder::new()
            .connect(self.rpc_url.as_str())
            .await
            .map_err(|e| {
                McpError::internal_error(format!("Failed to connect to RPC: {}", e), None)
            })?;

        if let Some(token_addr) = token_address {
            // 查询 ERC20 token 余额
            let token_address = Address::from_str(token_addr.as_str()).map_err(|e| {
                McpError::invalid_params(format!("Invalid token address: {}", e), None)
            })?;
            let erc20 = IERC20::new(token_address, provider);
            let balance = erc20.balanceOf(address).call().await.map_err(|e| {
                McpError::internal_error(format!("Failed to get token balance: {}", e), None)
            })?;
            let decimals = erc20.decimals().call().await.map_err(|e| {
                McpError::internal_error(format!("Failed to get token decimals: {}", e), None)
            })?;
            let symbol = erc20.symbol().call().await.map_err(|e| {
                McpError::internal_error(format!("Failed to get token symbol: {}", e), None)
            })?;
            Ok(CallToolResult::success(vec![Content::text(
                GetBalanceResponse {
                    price: MetaData {
                        value: balance,
                        decimals,
                        symbol,
                    },
                }
                .to_string(),
            )]))
        } else {
            // 查询 ETH 余额
            let balance_wei = provider.get_balance(address).await.map_err(|e| {
                McpError::internal_error(format!("Failed to get ETH balance: {}", e), None)
            })?;
            Ok(CallToolResult::success(vec![Content::text(
                GetBalanceResponse {
                    price: MetaData {
                        value: balance_wei,
                        decimals: 18,
                        symbol: "ETH".to_string(),
                    },
                }
                .to_string(),
            )]))
        }
    }
}
