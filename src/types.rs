use serde::{Deserialize, Serialize};
use alloy::primitives::U256;
use alloy::primitives::utils::format_units;
use std::fmt::{self, Display, Formatter};

/// 1. 余额查询
#[derive(Debug, Serialize, Deserialize)]
pub struct GetBalanceRequest {
    pub wallet_address: String,
    pub token_address: Option<String>, // None 表示查 ETH
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBalanceResponse {
    pub price: MetaData
}

/// 2. 价格查询
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTokenPriceRequest {
    pub token: String, // 地址
    pub fee: Option<u32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTokenPriceResponse {
    pub price: MetaData,
}

/// 3. 兑换模拟
#[derive(Debug, Serialize, Deserialize)]
pub struct SwapTokensRequest {
    pub from_token: String, // 地址或符号
    pub to_token: String,
    pub amount: f64, // 人类可读，例如 "0.5"
    pub slippage_pct: u128,
    pub fee: Option<u32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapTokensResponse {
    pub estimated_out: MetaData, // 预计到手数量（已格式化）
    pub gas_price: String,     // Gwei 字符串
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    pub value: U256,
    pub decimals: u8,
    pub symbol: String,
}


impl Display for MetaData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let format = format_units(self.value, self.decimals)
            .unwrap_or_else(|_| "0".to_string());
        write!(f, "{} {}", format, self.symbol)
    }
}



// 1. 余额
impl Display for GetBalanceResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.price)
    }
}

// 2. 价格（统一 6 位小数）
impl Display for GetTokenPriceResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.price)
    }
}

// 3. 兑换结果
impl Display for SwapTokensResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "estimated_out: {} | gas_price: {} Gwei",
            self.estimated_out,
            self.gas_price,
        )
    }
}
