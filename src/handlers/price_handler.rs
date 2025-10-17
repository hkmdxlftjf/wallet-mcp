use crate::models::{QUOTER_ADDRESS, USDT_ADDRESS};
use crate::models::{IQuoter, IERC20};
use crate::models::{GetTokenPriceRequest, GetTokenPriceResponse, MetaData};
use alloy::primitives::{Address, U160, U256};
use alloy::primitives::aliases::U24;
use alloy::providers::ProviderBuilder;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{model::*, ErrorData as McpError};
use std::str::FromStr;

#[derive(Clone)]
pub struct PriceHandler {
    rpc_url: String,
}

impl PriceHandler {
    pub fn new(rpc_url: String) -> Self {
        Self { rpc_url }
    }

    pub async fn handle_get_token_price(
        &self,
        Parameters(GetTokenPriceRequest { token, fee }): Parameters<GetTokenPriceRequest>,
    ) -> Result<CallToolResult, McpError> {
        let provider = ProviderBuilder::new()
            .connect(&self.rpc_url)
            .await
            .map_err(|e| {
                McpError::internal_error(format!("Failed to connect to RPC: {}", e), None)
            })?;
        let token_address = Address::from_str(token.as_str())
            .map_err(|e| McpError::invalid_params(format!("Invalid token address: {}", e), None))?;
        // let quote = IQuoterV2::new(QUOTER_ADDRESS, provider.clone());
        let quoter = IQuoter::new(QUOTER_ADDRESS, provider.clone());
        let token_contract = IERC20::new(token_address, provider.clone());
        let decimals = token_contract.decimals().call().await.map_err(|e| {
            McpError::internal_error(format!("Failed to get token decimals: {}", e), None)
        })?;
        let one_token = U256::from(10).pow(U256::from(decimals));

        let data = quoter
            .quoteExactInputSingle(
                token_address,
                USDT_ADDRESS,
                U24::from(fee.unwrap_or(3000)),
                one_token,
                U160::ZERO,
            )
            .call()
            .await
            .map_err(|e| {
                McpError::internal_error(format!("Failed to get token price quote: {}", e), None)
            })?;
        // let usd_out = quote.quoteExactInputSingle()

        Ok(CallToolResult::success(vec![Content::text(
            GetTokenPriceResponse {
                price: MetaData {
                    value: data,
                    decimals: 6,
                    symbol: "USDT".to_string(),
                },
            }
            .to_string(),
        )]))
    }
}
