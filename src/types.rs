use serde::{Deserialize, Serialize};
use std::str::FromStr;
use alloy::primitives::U256;
use alloy::transports::TransportError;
use anyhow::Error;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
}

impl From<TransportError> for RpcError {
    fn from(e: TransportError) -> Self {
        RpcError {
            code: -32603, // JSON-RPC internal
            message: format!("Transport: {}", e),
        }
    }
}

/// 1. 余额查询
#[derive(Debug, Serialize, Deserialize)]
pub struct GetBalanceRequest {
    pub wallet_address: String,
    pub token_address: Option<String>, // None 表示查 ETH
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBalanceResponse {
    pub balance: U256, // 已按 decimals 格式化后的字符串，例如 "1.23"
    pub decimals: u8,
    pub symbol: String,
}

/// 2. 价格查询
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTokenPriceRequest {
    pub token: String, // 地址或符号，如 "0xA0b869..." 或 "USDT"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTokenPriceResponse {
    pub price: U256,      // 人类可读，例如 "1850.42"
    pub decimals: u8,
    pub symbol: String,
}

/// 3. 兑换模拟
#[derive(Debug, Serialize, Deserialize)]
pub struct SwapTokensRequest {
    pub from_token: String, // 地址或符号
    pub to_token: String,
    pub amount: String, // 人类可读，例如 "0.5"
    pub slippage_pct: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapTokensResponse {
    pub estimated_out: String, // 预计到手数量（已格式化）
    pub gas_limit: String,     // 16 进制或 10 进制字符串
    pub gas_price: String,     // Gwei 字符串
    pub to: String,            // 目标合约地址
    pub data: String,          // 已编码的 calldata
}


use alloy::primitives::utils::format_units;
use std::fmt::{self, Display, Formatter};
use alloy::hex;

// 1. 余额
impl Display for GetBalanceResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let amount = format_units(self.balance, self.decimals)
            .unwrap_or_else(|_| "0".to_string());
        write!(f, "{} {}", amount, self.symbol)
    }
}

// 2. 价格（统一 6 位小数）
impl Display for GetTokenPriceResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let price = format_units(self.price, 6)
            .unwrap_or_else(|_| "0".to_string());
        write!(f, "{} {} ({} decimals)", price, self.symbol, self.decimals)
    }
}

// 3. 兑换结果
impl Display for SwapTokensResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "estimated_out: {} | gas_limit: {} | gas_price: {} Gwei | to: {} | data: 0x{}",
            self.estimated_out,
            self.gas_limit,
            self.gas_price,
            self.to,
            hex::encode(&self.data)
        )
    }
}

// 4. 错误
impl Display for RpcError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "RPC error {}: {}", self.code, self.message)
    }
}