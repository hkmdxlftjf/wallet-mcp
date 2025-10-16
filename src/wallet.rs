use alloy::primitives::aliases::U24;
use alloy::primitives::utils::{ parse_units};
use alloy::primitives::{Address,  U160, U256};
use anyhow::{Error, Result};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use crate::constants::{ QUOTERV2_ADDRESS, QUOTER_ADDRESS, ROUTER_ADDRESS, USDT_ADDRESS};
use crate::sol::{IQuoter, IQuoterV2, ISwapRouter, IERC20};
use crate::types::{GetBalanceRequest, GetBalanceResponse, GetTokenPriceRequest, GetTokenPriceResponse, MetaData, SwapTokensRequest, SwapTokensResponse};

pub struct WalletMcpServer {
    rpc_url: String,
    private_key: String,
}

impl WalletMcpServer {
    pub fn new(rpc_url: String, private_key: String) -> Self {
        Self {
            rpc_url,
            private_key,
        }
    }
}

/// 给 MCP-Server 暴露的 trait


impl WalletMcpServer {

    pub async fn get_balance(&self, req: GetBalanceRequest) -> Result<GetBalanceResponse> {
        let address = Address::from_str(req.wallet_address.as_str())?;
        let provider = ProviderBuilder::new()
            .connect(self.rpc_url.as_str())
            .await?;

        if let Some(token_addr) = req.token_address {
            // 查询 ERC20 token 余额
            let token_address = Address::from_str(token_addr.as_str())?;
            let erc20 = IERC20::new(token_address, provider);
            let balance = erc20.balanceOf(address).call().await?;
            let decimals = erc20.decimals().call().await?;
            let symbol = erc20.symbol().call().await?;
            Ok(GetBalanceResponse {
                price:MetaData {
                    value: balance,
                    decimals,
                    symbol,
                }
            })
        } else {
            // 查询 ETH 余额
            let balance_wei = provider.get_balance(address).await?;
            Ok(GetBalanceResponse {
                price:MetaData{
                value: balance_wei,
                decimals: 18,
                symbol: "ETH".to_string(),
                }
            })
        }
    }

    pub async fn get_token_price(&self, req: GetTokenPriceRequest) -> Result<GetTokenPriceResponse, Error> {
        let provider = ProviderBuilder::new()
            .connect(&self.rpc_url)
            .await?;
        let token_address = Address::from_str(req.token.as_str())?;
        // let quote = IQuoterV2::new(QUOTER_ADDRESS, provider.clone());
        let quoter = IQuoter::new(QUOTER_ADDRESS, provider.clone());
        let token_contract = IERC20::new(token_address, provider.clone());
        let decimals = token_contract.decimals().call().await?;
        let one_token = U256::from(10).pow(U256::from(decimals));

        let data = quoter.quoteExactInputSingle(
            token_address,
            USDT_ADDRESS,
            U24::from(req.fee.unwrap_or(3000)),
            one_token,
            U160::ZERO
        ).call().await?;
        // let usd_out = quote.quoteExactInputSingle()
        Ok(GetTokenPriceResponse{
            price:MetaData {
                value: data,
                decimals: 6,
                symbol: "USDT".to_string(),
            }
        })
    }

    pub async fn swap_tokens(&self, req: SwapTokensRequest) -> Result<SwapTokensResponse> {
        let signer: PrivateKeySigner = self.private_key.parse()?;
        let provider = ProviderBuilder::new()
            .connect(self.rpc_url.as_str())
            .await?;

        // 获取代币信息
        let from_token_address = Address::from_str(req.from_token.as_str())?;
        let to_token_address = Address::from_str(req.to_token.as_str())?;


        let from_token_contract = IERC20::new(from_token_address, provider.clone());
        let from_token_decimals = from_token_contract.decimals().call().await?;

        let amount_in = parse_units(req.amount.to_string().as_str(), from_token_decimals)?;
        // let amountIn = U256::from(req.amount) * U256::from(10).pow(U256::from(decimals));
        println!("amount is {}", amount_in.to_string());
        let quoter_v2 = IQuoterV2::new(QUOTERV2_ADDRESS, provider.clone());
        let params = IQuoterV2::QuoteExactInputSingleParams{
            tokenIn: from_token_address,
            tokenOut: to_token_address,
            amountIn: amount_in.into(),
            fee: U24::from(req.fee.unwrap_or(3000)),
            sqrtPriceLimitX96: U160::ZERO
        };
        let res = quoter_v2.quoteExactInputSingle(params).call().await?;


        // 根据滑点计算 amountOutMinimum
        let slippage_multiplier = 10000 - req.slippage_pct;
        let amount_out_min = res.amountOut * U256::from(slippage_multiplier) / U256::from(10000);
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
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
        let decimals = to_token_contract.decimals().call().await?;
        let symbol = to_token_contract.symbol().call().await?;

        let _ = from_token_contract.approve(ROUTER_ADDRESS, amount_in.into()).from(signer.address()).send().await?;

        let router = ISwapRouter::new(ROUTER_ADDRESS, provider.clone());
        let value = router.exactInputSingle(swap_params.clone()).from(signer.address()).call().await?;
        let gas_estimate = router.exactInputSingle(swap_params.clone()).from(signer.address()).call().await?;
        // let data = router.exactInputSingle(swap_params).from(signer.address()).from(signer.address()).send().await?;
        Ok(SwapTokensResponse{
            estimated_out: MetaData{
                value,
                decimals,
                symbol,
            },
            gas_price: gas_estimate.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use crate::constants::{BTC_ADDRESS, WETH_ADDRESS};
    use super::*;
    async fn set_up() -> Result<WalletMcpServer,Error> {
        dotenvy::dotenv()?;
        let rpc_url = env::var("ETH_RPC_URL").unwrap_or_else(|_| "default_key".to_string());
        let private_key = env::var("PRIVATE_KEY").unwrap_or_else(|_| "default_key".to_string());
        let wallet = WalletMcpServer::new(rpc_url, private_key);
        Ok(wallet)
    }

    #[tokio::test]
    async fn test_get_balance() -> Result<()>{
        let wallet = set_up().await?;
        let signer:PrivateKeySigner = wallet.private_key.parse()?;
        let balance_info = wallet.get_balance(GetBalanceRequest{
            wallet_address: signer.address().to_string(),
            token_address:None,
        }).await?;
        println!("{}",balance_info);
        assert_ne!(balance_info.price.value, U256::ZERO);
        let balance_info = wallet.get_balance(GetBalanceRequest{
            wallet_address: signer.address().to_string(),
            token_address:Some(WETH_ADDRESS.to_string()),
        }).await?;
        println!("{}",balance_info);
        assert_ne!(balance_info.price.value, U256::ZERO);
        let balance_info = wallet.get_balance(GetBalanceRequest{
            wallet_address: signer.address().to_string(),
            token_address:Some(BTC_ADDRESS.to_string()),
        }).await?;
        println!("{}",balance_info);
        assert_ne!(balance_info.price.value, U256::ZERO);
        let balance_info = wallet.get_balance(GetBalanceRequest{
            wallet_address: signer.address().to_string(),
            token_address:Some(USDT_ADDRESS.to_string()),
        }).await?;
        println!("{}",balance_info);
        // assert_ne!(balanceInfo.unwrap().balance, U256::ZERO);
        Ok(())
    }


    #[tokio::test]
    async fn test_get_token_price() -> Result<(), Error> {
        let wallet = set_up().await?;

        let req = GetTokenPriceRequest {
            token: WETH_ADDRESS.to_string(),
            fee: None,
        };

        let resp = wallet.get_token_price(req).await?;
        println!("get token price result: {}", resp);
        Ok(())
    }
    #[tokio::test]
    async fn test_swap_tokens() -> Result<(), Error> {
        let wallet = set_up().await?;

        let req = SwapTokensRequest {
            from_token: WETH_ADDRESS.to_string(),
            to_token: USDT_ADDRESS.to_string(),
            amount: 0.5,
            slippage_pct: 500,
            fee: Some(10000),
        };

        let resp = wallet.swap_tokens(req).await?;
        println!("{}",resp);

        let req = SwapTokensRequest {
            from_token: BTC_ADDRESS.to_string(),
            to_token: USDT_ADDRESS.to_string(),
            amount: 0.1,
            slippage_pct: 500,
            fee: Some(10000),
        };

        let resp = wallet.swap_tokens(req).await?;
        println!("{}",resp);

        Ok(())
    }
}
