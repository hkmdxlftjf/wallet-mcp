use crate::models::{IQuoterV2, ISwapRouter, IERC20};
use crate::models::{MetaData, SwapTokensRequest, SwapTokensResponse};
use crate::models::{QUOTERV2_ADDRESS, ROUTER_ADDRESS};
use alloy::primitives::aliases::U24;
use alloy::primitives::utils::parse_units;
use alloy::primitives::{Address, U160, U256};
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{model::*, ErrorData as McpError};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct SwapHandler {
    private_key: String,
    rpc_url: String,
}

impl SwapHandler {
    pub fn new(private_key: String, rpc_url: String) -> Self {
        Self {
            private_key,
            rpc_url,
        }
    }

    pub async fn handle_swap_tokens(
        &self,
        Parameters(SwapTokensRequest {
            from_token,
            to_token,
            amount,
            slippage_pct,
            fee,
        }): Parameters<SwapTokensRequest>,
    ) -> Result<CallToolResult, McpError> {
        // 验证私钥格式
        let signer: PrivateKeySigner = self
            .private_key
            .parse()
            .map_err(|e| McpError::invalid_params(format!("Invalid private key: {}", e), None))?;

        // 连接到提供者
        let provider = ProviderBuilder::new()
            .connect(self.rpc_url.as_str())
            .await
            .map_err(|e| {
                McpError::internal_error(format!("Failed to connect to RPC: {}", e), None)
            })?;

        // 验证代币地址格式
        let from_token_address = Address::from_str(from_token.as_str()).map_err(|e| {
            McpError::invalid_params(
                format!("Invalid from_token address '{}': {}", from_token, e),
                None,
            )
        })?;
        let to_token_address = Address::from_str(to_token.as_str()).map_err(|e| {
            McpError::invalid_params(
                format!("Invalid to_token address '{}': {}", to_token, e),
                None,
            )
        })?;

        // 检查是否为相同代币
        if from_token_address == to_token_address {
            return Err(McpError::invalid_params("Cannot swap the same token", None));
        }

        // 获取代币信息
        let from_token_contract = IERC20::new(from_token_address, provider.clone());
        let from_token_decimals = from_token_contract.decimals().call().await.map_err(|e| {
            McpError::internal_error(format!("Failed to get from_token decimals: {}", e), None)
        })?;

        // 解析金额
        let amount_in: U256 = parse_units(amount.to_string().as_str(), from_token_decimals)
            .map_err(|e| {
                McpError::invalid_params(format!("Invalid amount '{}': {}", amount, e), None)
            })?
            .into();

        // 验证金额不为零
        if amount_in == U256::ZERO {
            return Err(McpError::invalid_params("Amount cannot be zero", None));
        }

        // 获取报价
        let quoter_v2 = IQuoterV2::new(QUOTERV2_ADDRESS, provider.clone());
        let params = IQuoterV2::QuoteExactInputSingleParams {
            tokenIn: from_token_address,
            tokenOut: to_token_address,
            amountIn: amount_in.into(),
            fee: U24::from(fee.unwrap_or(3000)),
            sqrtPriceLimitX96: U160::ZERO,
        };

        let res = quoter_v2
            .quoteExactInputSingle(params)
            .call()
            .await
            .map_err(|e| McpError::internal_error(format!("Failed to get quote: {}", e), None))?;

        // 根据滑点计算 amountOutMinimum
        let slippage_multiplier = 10000 - slippage_pct;
        let amount_out_min = res.amountOut * U256::from(slippage_multiplier) / U256::from(10000);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| {
                McpError::internal_error(format!("Failed to get current time: {}", e), None)
            })?
            .as_secs();
        let deadline = now + 20 * 60; // 20 分钟有效期

        let swap_params = ISwapRouter::ExactInputSingleParams {
            tokenIn: from_token_address,
            tokenOut: to_token_address,
            fee: U24::from(3000),
            recipient: signer.address(),
            amountIn: amount_in.into(),
            deadline: U256::from(deadline),
            amountOutMinimum: amount_out_min,
            sqrtPriceLimitX96: U160::ZERO,
        };

        // 获取目标代币信息
        let to_token_contract = IERC20::new(to_token_address, provider.clone());
        let decimals = to_token_contract.decimals().call().await.map_err(|e| {
            McpError::internal_error(format!("Failed to get to_token decimals: {}", e), None)
        })?;
        let symbol = to_token_contract.symbol().call().await.map_err(|e| {
            McpError::internal_error(format!("Failed to get to_token symbol: {}", e), None)
        })?;

        // 模拟授权代币（不实际发送交易）
        // let approve_call = from_token_contract
        //     .approve(ROUTER_ADDRESS, amount_in.into())
        //     .from(signer.address());

        // // 只进行模拟调用，不发送实际交易
        // let _ = approve_call.call().await.map_err(|e| {
        //     McpError::internal_error(format!("Failed to simulate token approval: {}", e), None)
        // })?;

        // 执行交换模拟
        let router = ISwapRouter::new(ROUTER_ADDRESS, provider.clone());
        let value = router
            .exactInputSingle(swap_params.clone())
            .from(signer.address())
            .call()
            .await
            .map_err(|e| {
                McpError::internal_error(format!("Failed to simulate swap: {}", e), None)
            })?;
        let gas_estimate = router
            .exactInputSingle(swap_params.clone())
            .from(signer.address())
            .call()
            .await
            .map_err(|e| {
                McpError::internal_error(format!("Failed to estimate gas: {}", e), None)
            })?;

        Ok(CallToolResult::success(vec![Content::text(
            SwapTokensResponse {
                estimated_out: MetaData {
                    value,
                    decimals,
                    symbol,
                },
                gas_price: gas_estimate.to_string(),
            }
            .to_string(),
        )]))
    }
}
