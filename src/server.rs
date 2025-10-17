use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use alloy::primitives::{Address, U160, U256};
use alloy::primitives::aliases::U24;
use alloy::primitives::utils::parse_units;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use rmcp::{
    ErrorData as McpError, ServerHandler, handler::server::tool::ToolRouter, model::*, tool, tool_handler, tool_router,
};
use rmcp::handler::server::wrapper::Parameters;
use crate::constants::{QUOTERV2_ADDRESS, QUOTER_ADDRESS, ROUTER_ADDRESS, USDT_ADDRESS};
use crate::sol::{IQuoter, IQuoterV2, ISwapRouter, IERC20};
use crate::types::{GetBalanceRequest, GetBalanceResponse, GetTokenPriceRequest, GetTokenPriceResponse, MetaData, SwapTokensRequest, SwapTokensResponse};

#[derive(Clone)]
pub struct McpServer {
    private_key: String,
    rpc_url: String,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl McpServer {
    #[allow(dead_code)]
    pub fn new(private_key: String, rpc_url: String) -> Self {
        Self {
            tool_router: Self::tool_router(),
            private_key,
            rpc_url
        }
    }
    #[tool(description = "Query ETH and ERC20 token balances")]
    async fn get_balance(
        &self,
        Parameters(GetBalanceRequest { wallet_address, token_address }): Parameters<GetBalanceRequest>,
    ) -> Result<CallToolResult, McpError> {
        let address = Address::from_str(wallet_address.as_str());
        let provider = ProviderBuilder::new()
            .connect(self.rpc_url.as_str())
            .await;

        if let Some(token_addr) = token_address {
            // 查询 ERC20 token 余额
            let token_address = Address::from_str(token_addr.as_str());
            let erc20 = IERC20::new(token_address.unwrap(), provider.unwrap());
            let balance = erc20.balanceOf(address.unwrap()).call().await;
            let decimals = erc20.decimals().call().await;
            let symbol = erc20.symbol().call().await;
            Ok(CallToolResult::success(vec![Content::text(
                GetBalanceResponse {
                    price: MetaData {
                        value: balance.unwrap(),
                        decimals: decimals.unwrap(),
                        symbol: symbol.unwrap(),
                    },
                }.to_string(),
            )]))
        } else {
            // 查询 ETH 余额
            let balance_wei = provider.unwrap().get_balance(address.unwrap()).await.unwrap();
            Ok(CallToolResult::success(vec![Content::text(
                GetBalanceResponse {
                    price: MetaData {
                        value: balance_wei,
                        decimals: 18,
                        symbol: "ETH".to_string(),
                    },
                }.to_string(),
            )]))
        }
    }

    #[tool(description = "Get current token price in USD or ETH")]
    async fn get_token_price(
        &self,
        Parameters(GetTokenPriceRequest { token, fee }): Parameters<GetTokenPriceRequest>,
    ) -> Result<CallToolResult, McpError> {
        let provider = ProviderBuilder::new().connect(&self.rpc_url).await.unwrap();
        let token_address = Address::from_str(token.as_str()).unwrap();
        // let quote = IQuoterV2::new(QUOTER_ADDRESS, provider.clone());
        let quoter = IQuoter::new(QUOTER_ADDRESS, provider.clone());
        let token_contract = IERC20::new(token_address, provider.clone());
        let decimals = token_contract.decimals().call().await.unwrap();
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
            .await;
        // let usd_out = quote.quoteExactInputSingle()

        Ok(CallToolResult::success(vec![Content::text(
            GetTokenPriceResponse {
                price: MetaData {
                    value: data.unwrap(),
                    decimals: 6,
                    symbol: "USDT".to_string(),
                },
            }.to_string(),
        )]))
    }



    #[tool(description = "Execute a token swap on Uniswap V2 or V3")]
    async fn swap_tokens(
        &self,
        Parameters(SwapTokensRequest { from_token, to_token, amount, slippage_pct, fee }): Parameters<SwapTokensRequest>,
    ) -> Result<CallToolResult, McpError> {
        let signer: PrivateKeySigner = self.private_key.parse().unwrap();
        let provider = ProviderBuilder::new()
            .connect(self.rpc_url.as_str())
            .await.unwrap();

        // 获取代币信息
        let from_token_address = Address::from_str(from_token.as_str()).unwrap();
        let to_token_address = Address::from_str(to_token.as_str()).unwrap();

        let from_token_contract = IERC20::new(from_token_address, provider.clone());
        let from_token_decimals = from_token_contract.decimals().call().await.unwrap();

        let amount_in = parse_units(amount.to_string().as_str(), from_token_decimals).unwrap();
        // let amountIn = U256::from(req.amount) * U256::from(10).pow(U256::from(decimals));
        let quoter_v2 = IQuoterV2::new(QUOTERV2_ADDRESS, provider.clone());
        let params = IQuoterV2::QuoteExactInputSingleParams {
            tokenIn: from_token_address,
            tokenOut: to_token_address,
            amountIn: amount_in.into(),
            fee: U24::from(fee.unwrap_or(3000)),
            sqrtPriceLimitX96: U160::ZERO,
        };
        let res = quoter_v2.quoteExactInputSingle(params).call().await.unwrap();

        // 根据滑点计算 amountOutMinimum
        let slippage_multiplier = 10000 - slippage_pct;
        let amount_out_min = res.amountOut * U256::from(slippage_multiplier) / U256::from(10000);
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
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

        let to_token_contract = IERC20::new(to_token_address, provider.clone());
        let decimals = to_token_contract.decimals().call().await.unwrap();
        let symbol = to_token_contract.symbol().call().await.unwrap();

        let _ = from_token_contract
            .approve(ROUTER_ADDRESS, amount_in.into())
            .from(signer.address())
            .send()
            .await.unwrap();

        let router = ISwapRouter::new(ROUTER_ADDRESS, provider.clone());
        let value = router
            .exactInputSingle(swap_params.clone())
            .from(signer.address())
            .call()
            .await.unwrap();
        let gas_estimate = router
            .exactInputSingle(swap_params.clone())
            .from(signer.address())
            .call()
            .await.unwrap();
        // let data = router.exactInputSingle(swap_params).from(signer.address()).from(signer.address()).send().await?;
        Ok(CallToolResult::success(vec![Content::text(
            SwapTokensResponse {
                estimated_out: MetaData {
                    value,
                    decimals,
                    symbol,
                },
                gas_price: gas_estimate.to_string(),
            }.to_string(),
        )]))
    }



}

#[tool_handler]
impl ServerHandler for McpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides wallet tools. Tools: get_balance.".to_string()),
        }
    }
}
