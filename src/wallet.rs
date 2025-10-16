use std::ops::Div;
use alloy::primitives::aliases::U24;
use alloy::primitives::utils::{ parse_units};
use alloy::primitives::{Address,  U160, U256};
use anyhow::{Error, Result};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use crate::constants::{FACTORY_ADDRESS, QUOTER_ADDRESS, ROUTER_ADDRESS, USDT_ADDRESS};
use crate::sol::{IQuoter, ISwapRouter, IUniswapV3Factory, IUniswapV3Pool, IERC20};
use crate::types::{GetBalanceRequest, GetBalanceResponse, GetTokenPriceRequest, GetTokenPriceResponse, SwapTokensRequest, SwapTokensResponse};

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
                balance,
                decimals,
                symbol,
            })
        } else {
            // 查询 ETH 余额
            let balance_wei = provider.get_balance(address).await?;
            Ok(GetBalanceResponse {
                balance: balance_wei,
                decimals: 18,
                symbol: "ETH".to_string(),
            })
        }
    }

    pub async fn get_token_price(&self, req: GetTokenPriceRequest) -> Result<GetTokenPriceResponse, Error> {
        let provider = ProviderBuilder::new()
            .connect(&self.rpc_url)
            .await?;
        let quote_address = Address::from_str(req.token.as_str())?;
        // 如果 Chainlink 没有对应的 feed，回退到 Uniswap V3
        let factory = IUniswapV3Factory::new(FACTORY_ADDRESS, provider.clone());

        // 获取 WETH/USDT 的 0.05% 池（500）
        let pool_address = match factory
            .getPool(quote_address, USDT_ADDRESS, U24::from(500))
            .call()
            .await
        {
            Ok(addr) => addr,
            Err(e) => {
                eprintln!("getPool failed: {e}");
                println!("getPool failed: {e}");
                // 这里可以返回 Error，或给一个默认零地址，按业务需要处理
                return Err(e.into()); // 或 Err(Error::PoolNotFound)
            }
        };

        println!("pool address {}", pool_address.to_string());

        if pool_address.is_zero() {
            anyhow::bail!("No V3 pool exists for this pair");
        }

        let pool = IUniswapV3Pool::new(pool_address, provider.clone());
        let slot0 = pool
            .slot0()
            .call()
            .await
            .map_err(|e| {
                eprintln!("slot0 call failed: {e}");
                e
            })?;
        let sqrt_price_x96 = slot0.sqrtPriceX96;
        println!("sqrt_price_x96: {}", sqrt_price_x96);
        let token0 = pool.token0().call().await?;
        let token1 = pool.token1().call().await?;
        println!("token0: {}", token0);
        println!("token1: {}", token1);
        let dec0 = IERC20::new(token0, provider.clone())
            .decimals()
            .call()
            .await?;
        let dec1 = IERC20::new(token1, provider.clone())
            .decimals()
            .call()
            .await?;
        println!("dec0: {}", dec0);
        println!("dec1: {}", dec1);

        let mut price = price_from_sqrt(sqrt_price_x96, dec0, dec1);
        println!("price: {}", price);
        // 如果 token0 是 USDT，方向反转
        if token0 == USDT_ADDRESS {
            price = U256::from(10).pow(U256::from(2 * dec0)) / price;
            println!("price: {}", price);
            return Ok((GetTokenPriceResponse{
                price,
                decimals:dec0,
                symbol: "USDT".to_string(),
            }))
        }
        Ok((GetTokenPriceResponse{
            price,
            decimals:dec1,
            symbol: "USDT".to_string(),
        }))
    }

    pub async fn swap_tokens(&self, req: SwapTokensRequest) -> Result<SwapTokensResponse> {
        let signer: PrivateKeySigner = self.private_key.parse()?;
        let provider = ProviderBuilder::new()
            .wallet(signer.clone())
            .connect(self.rpc_url.as_str())
            .await?;

        // 获取代币信息
        let from_token_address = Address::from_str(req.from_token.as_str())?;
        let to_token_address = Address::from_str(req.to_token.as_str())?;

        let from_token_decimals = IERC20::new(from_token_address, provider.clone())
            .decimals()
            .call()
            .await?;
        let from_token_symbol = IERC20::new(from_token_address, provider.clone())
            .symbol()
            .call()
            .await?;
        let to_token_symbol = IERC20::new(to_token_address, provider.clone())
            .symbol()
            .call()
            .await?;
        let to_token_decimals = IERC20::new(to_token_address, provider.clone())
            .decimals()
            .call()
            .await?;

        // 将用户输入的金额转换为最小单位
        let amount_in_wei = parse_units(&req.amount, from_token_decimals)?.into();

        // 使用 Quoter 获取预估输出 - 这是安全的只读调用
        let quoter = IQuoter::new(QUOTER_ADDRESS, provider.clone());
        let estimated_amount_out = quoter
            .quoteExactInputSingle(
                from_token_address,
                to_token_address,
                U24::from(3000), // 0.3% fee tier
                amount_in_wei,
                U160::ZERO,
            )
            .call()
            .await?;

        // 计算滑点保护的最小输出金额
        let slippage_percent: f64 = req.slippage_pct;
        let slippage_multiplier = (100.0 - slippage_percent) / 100.0;
        let min_amount_out =
            (estimated_amount_out.to::<u128>() as f64 * slippage_multiplier) as u128;

        // 构建交易参数用于 gas 估算
        let router = ISwapRouter::new(ROUTER_ADDRESS, provider.clone());
        let deadline = U256::from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 1200,
        ); // 20分钟后过期

        let swap_params = ISwapRouter::ExactInputSingleParams {
            tokenIn: from_token_address,
            tokenOut: to_token_address,
            fee: U24::from(3000),
            recipient: signer.address(), // 使用有效地址进行模拟
            deadline,
            amountIn: amount_in_wei,
            amountOutMinimum: U256::from(min_amount_out),
            sqrtPriceLimitX96: U160::ZERO,
        };
        // let data = router.exactInputSingle(swap_params).call().await?;
        let gas_estimate = router
            .exactInputSingle(swap_params.clone())
            .from(Address::from_str("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266")?)
            .estimate_gas()
            .await?;
        println!("data is: {}", gas_estimate);
        // 智能 gas 估算 - 基于交易复杂度
        let result = "";
        println!("{}", result);
        Ok(SwapTokensResponse{
            estimated_out: gas_estimate.to_string(),
            gas_limit: gas_estimate.to_string(),
            gas_price: gas_estimate.to_string(),
            to: req.to_token.to_string(),
            data: "".to_string()
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
        Ok((wallet))
    }

    #[tokio::test]
    async fn test_get_balance() -> Result<()>{
        let wallet = set_up().await?;
        let signer:PrivateKeySigner = wallet.private_key.parse()?;
        let balanceInfo = wallet.get_balance(GetBalanceRequest{
            wallet_address: signer.address().to_string(),
            token_address:None,
        }).await?;
        println!("{}",balanceInfo);
        assert_ne!(balanceInfo.balance, U256::ZERO);
        let balanceInfo = wallet.get_balance(GetBalanceRequest{
            wallet_address: signer.address().to_string(),
            token_address:Some(WETH_ADDRESS.to_string()),
        }).await?;
        println!("{}",balanceInfo);
        assert_ne!(balanceInfo.balance, U256::ZERO);
        let balanceInfo = wallet.get_balance(GetBalanceRequest{
            wallet_address: signer.address().to_string(),
            token_address:Some(BTC_ADDRESS.to_string()),
        }).await?;
        println!("{}",balanceInfo);
        assert_ne!(balanceInfo.balance, U256::ZERO);
        let balanceInfo = wallet.get_balance(GetBalanceRequest{
            wallet_address: signer.address().to_string(),
            token_address:Some(USDT_ADDRESS.to_string()),
        }).await.ok();
        println!("{}",balanceInfo.unwrap());
        // assert_ne!(balanceInfo.unwrap().balance, U256::ZERO);
        Ok(())
    }


    #[tokio::test]
    async fn test_get_token_price_usdt() -> Result<(), anyhow::Error> {
        let wallet = set_up().await?;

        let req = GetTokenPriceRequest {
            token: WETH_ADDRESS.to_string(),
        };

        let resp = wallet.get_token_price(req).await?;

        println!("Price: {}", resp.price);
        println!("Decimals: {}", resp.decimals);
        println!("Symbol: {}", resp.symbol);

        Ok(())
    }
}


/// sqrtPriceX96 → 价格计算函数
/// 返回 1 个 token0 对应的 token1 价格，以 U256 表示
/// 精度：18 位小数（如果想 6 位，调用处再 / 1e12 即可）
fn price_from_sqrt(sqrt_price_x96: U160, d0: u8, d1: u8) -> U256 {
    // 1. sqrtPriceX96 转为 U256
    let sqrt = U256::from(sqrt_price_x96);

    // 2. 计算 sqrt² / 2¹⁹² ，先放大 1e18 保证精度
    let two192 = U256::from(2).pow(U256::from(192));
    let price_e18 = sqrt * sqrt * U256::from(10).pow(U256::from(18)) / two192;

    // 3. 处理小数位差 10^(d0-d1)
    let shift = d0 as i32 - d1 as i32;
    let price = if shift >= 0 {
        price_e18 * U256::from(10).pow(U256::from(shift as u64))
    } else {
        price_e18 / U256::from(10).pow(U256::from(-shift as u64))
    };

    price
}