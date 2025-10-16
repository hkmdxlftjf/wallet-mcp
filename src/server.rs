use alloy::primitives::Address;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use serde::{Deserialize, Serialize};
use alloy::providers::{Provider};
use anyhow::Result;
use crate::wallet::{SwapParams, Wallet};

#[derive(Debug, Deserialize)]
pub struct Request {
    pub jsonrpc: String,
    pub method: String,
    pub id: Option<serde_json::Value>,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub jsonrpc: &'static str,
    pub result: serde_json::Value,
    pub id: serde_json::Value,
}

pub struct Server {}

impl Server {
    pub async fn new() -> Result<Self> {
        Ok(Self{})
    }

    pub async fn run(&self, rpc_url: String) -> anyhow::Result<()> {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut writer = stdout;
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        let wallet = Wallet::new(rpc_url);
        while let Some(line) = lines.next_line().await? {
            if line.trim().is_empty() { continue; }

            let req: Request = match serde_json::from_str(&line) {
                Ok(r) => r,
                Err(e) => { eprintln!("Invalid JSON: {}", e); continue; }
            };

            let result = match req.method.as_str() {
                "ping" => serde_json::json!({"message": "pong"}),
                "get_balance" => {
                    if let Some(params) = req.params.clone() {
                        if let Some(addr) = params.get("address").and_then(|v| v.as_str()) {
                            match wallet.get_balance(addr).await {
                                Ok(b) => serde_json::json!({"balance": b}),
                                Err(e) => serde_json::json!({"error": e.to_string()}),
                            }
                        } else {
                            serde_json::json!({"error": "missing address"})
                        }
                    } else {
                        serde_json::json!({"error": "missing params"})
                    }
                },
                "get_token_price" => {
                    if let Some(params) = req.params.clone() {
                        if let Some(addr) = params.get("quote_address").and_then(|v| v.as_str()) {
                            match wallet.get_token_price(addr.to_string()).await {
                                Ok(b) => serde_json::json!({"balance": b}),
                                Err(e) => serde_json::json!({"error": e.to_string()}),
                            }
                        } else {
                            serde_json::json!({"error": "missing address"})
                        }
                    } else {
                        serde_json::json!({"error": "missing params"})
                    }
                },
                // Handler
                "swap_tokens" => {
                    // 反序列化参数
                    match serde_json::from_value::<SwapParams>(req.params.clone().unwrap_or_default()) {
                        Ok(params) => {
                            // 直接传给 wallet.swap_tokens
                            match wallet.swap_tokens(params).await {
                                Ok(res) => serde_json::json!({"result": res}),
                                Err(e) => serde_json::json!({"error": e.to_string()}),
                            }
                        }
                        Err(_) => serde_json::json!({"error": "invalid parameters"}),
                    }
                }
                _ => serde_json::json!({"error": "unknown method"}),
            };

            if let Some(id) = req.id {
                let resp = Response {
                    jsonrpc: "2.0",
                    result,
                    id,
                };
                let resp_str = serde_json::to_string(&resp)?;
                writer.write_all(resp_str.as_bytes()).await?;
                writer.write_all(b"\n").await?;
                writer.flush().await?;
            }
        }

        Ok(())
    }
}
