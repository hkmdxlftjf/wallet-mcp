use alloy::providers::{Provider, ProviderBuilder};
use alloy::primitives::{address, Address, I256, U160, U256};
use std::str::FromStr;
use alloy::primitives::aliases::U24;
use alloy::primitives::utils::{format_ether, format_units, parse_units};
use alloy::sol;
use anyhow::Result;
use serde::Deserialize;
use std::ops::{Div, Mul};

pub struct Wallet {
    rpc_url: String,
}

#[derive(Deserialize)]
pub struct SwapParams {
    from_token: String,
    to_token: String,
    amount: String,
    #[serde(default = "default_slippage")]
    slippage: String,
}

fn default_slippage() -> String {
    "0.5".to_string()
}



const USDT_ADDRESS: Address = address!("0xdAC17F958D2ee523a2206206994597C13D831ec7");
const FACTORY_ADDRESS: Address = address!("0x1F98431c8aD98523631AE4a59f267346ea31F984");
const WETH_ADDRESS: Address = address!("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
const FEED_FACTORY_ADDRESS: Address = address!("0x47Fb2585D2C56Fe188D0E6ec628a38b74fCeeeDf");
const ROUTER_ADDRESS: Address = address!("E592427A0AEce92De3Edee1F18E0157C05861564"); // V3 Router
const QUOTER_ADDRESS: Address = address!("0xb27308f9F90D607463bb33eA1BeBb41C27CE5AB6"); // V3 Quoter


// Generate bindings for the AggregatorV3Interface contract
sol! {
    #[sol(rpc)]
    contract IUniswapV3Factory{
        function getPool(address tokenA, address tokenB, uint24 fee) external view returns (address pool);
    }
    #[sol(rpc)]
    contract IUniswapV3Pool{
        function slot0() external view returns (uint160 sqrtPriceX96, int24 tick, uint16 obsIdx, uint16 obsCard, uint16 obsCardNext, uint8 feeProtocol, bool unlocked);
        function token0() external view returns (address);
        function token1() external view returns (address);
    }
    #[sol(rpc)]
    contract IERC20 {
        function decimals() external view returns (uint8);
        function symbol() external view returns (string memory);
    }
    #[sol(rpc)]
    interface AggregatorV3Interface {
        function latestRoundData()
            external
            view
            returns (
                uint80 roundId,
                int256 answer,
                uint256 startedAt,
                uint256 updatedAt,
                uint80 answeredInRound
            );
        function decimals() external view returns (uint8);
    }
    #[sol(rpc)]
    interface FeedRegistryInterface {
        function getFeed(address base, address quote) external view returns (address aggregator);
    }
    #[sol(rpc)]
    interface ISwapRouter {
        struct ExactInputSingleParams {
            address tokenIn;
            address tokenOut;
            uint24 fee;
            address recipient;
            uint256 deadline;
            uint256 amountIn;
            uint256 amountOutMinimum;
            uint160 sqrtPriceLimitX96;
        }
        function exactInputSingle(ExactInputSingleParams calldata params) external payable returns (uint256 amountOut);
    }
    #[sol(rpc)]
    interface IQuoter {
        function quoteExactInputSingle(address tokenIn,address tokenOut,uint24 fee,uint256 amountIn,uint160 sqrtPriceLimitX96) external returns (uint256 amountOut);
    }

}
impl Wallet {
    pub fn new(rpc_url: String) -> Self {
        Self {rpc_url}
    }

    pub async fn get_balance(
        &self,
        addr: &str,
    ) -> Result<String> {
        let address = Address::from_str(addr)?;
        let provider = ProviderBuilder::new()
            .connect(self.rpc_url.as_str())
            .await?;
        let balance_wei = provider.get_balance(address).await?;
        let balance_eth = format_ether(balance_wei);
        Ok(balance_eth)
    }

    pub async fn get_token_price(&self, quote: String) -> Result<String> {
        let provider = ProviderBuilder::new().connect(self.rpc_url.as_str()).await?;
        let factory = IUniswapV3Factory::new(FACTORY_ADDRESS, provider.clone());
        let quote_address = Address::from_str(quote.as_str())?;

        // 获取 WETH/USDT 的 0.05% 池（500）
        let pool_address = factory
            .getPool(quote_address, USDT_ADDRESS, U24::from(500))
            .call()
            .await?;

        if pool_address.const_is_zero() {
            anyhow::bail!("No V3 pool exists for this pair");
        }


        let pool = IUniswapV3Pool::new(pool_address, provider.clone());
        let slot0 = pool.slot0().call().await?;
        let sqrt_price_x96 = slot0.sqrtPriceX96;

        let token0 = pool.token0().call().await?;
        let token1 = pool.token1().call().await?;

        let dec0 = IERC20::new(token0, provider.clone()).decimals().call().await?;
        let dec1 = IERC20::new(token1, provider.clone()).decimals().call().await?;

        let mut price = price_from_sqrt(sqrt_price_x96.to::<u128>(), dec0, dec1);
        let mut quote_symbol = IERC20::new(token0,provider.clone()).symbol().call().await?;
        // 如果 token0 是 USDT，方向反转
        if token0 == USDT_ADDRESS {
            price = 1.0 / price;
            quote_symbol = IERC20::new(token1, provider.clone()).symbol().call().await?;
        }

        println!("1 {} = {:.2} USDT", quote_symbol, price);
        Ok(price.to_string())
    }

    pub async fn swap_tokens(
        &self,
        params: SwapParams
    ) -> Result<String> {
        // Implementation goes here
        let provider = ProviderBuilder::new().connect(self.rpc_url.as_str()).await?;
        let quoter = IQuoter::new(QUOTER_ADDRESS, provider.clone());
        let data = quoter.quoteExactInputSingle(
            Address::from_str(params.from_token.as_str())?,
            Address::from_str(params.to_token.as_str())?,
            U24::from(3000),
            U256::from(100_000_0000u128),
            U160::ZERO
        ).call().await?;
        let symbol = IERC20::new(Address::from_str(params.to_token.as_str())?, provider.clone()).symbol().call().await?;
        let decimals = IERC20::new(Address::from_str(params.to_token.as_str())?, provider.clone()).decimals().call().await?;
        println!("Estimated WETH out: {}", format_units(data, decimals)?);
        Ok("".to_string())
    }
    pub async fn get_feed(&self, quote: String) -> Option<Address> {
        let quote_address = Address::from_str(&*quote).ok()?; // 解析失败返回 None
        let provider = ProviderBuilder::new().connect(self.rpc_url.as_str()).await.ok()?; // 链接失败返回 None
        let feed_factory = FeedRegistryInterface::new(FEED_FACTORY_ADDRESS, provider.clone());
        // 查询 feed
        let feed_address = feed_factory.getFeed(USDT_ADDRESS, quote_address).call().await.ok()?;
        // println!("feed_address is {}",feed_address.ok()?.0);
        // 如果 feed_address 为零，返回 None
        // if feed_address.ok()?.is_zero() {
        //     return None;
        // } else {
        //     None
        // }
        Some(feed_address)
    }
}


/// sqrtPriceX96 → 价格计算函数
fn price_from_sqrt(sqrt: u128, d0: u8, d1: u8) -> f64 {
    let ratio = (sqrt as f64) / (2f64).powi(96);
    let price = ratio * ratio;
    let shift = d0 as i32 - d1 as i32;
    price * 10f64.powi(shift)
}