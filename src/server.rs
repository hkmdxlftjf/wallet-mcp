use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use crate::types::{GetBalanceRequest, GetTokenPriceRequest, SwapTokensRequest};
use crate::wallet::WalletMcpServer;

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
        Ok(Self {})
    }

    pub async fn run(&self, rpc_url: String, private_key: String) -> anyhow::Result<()> {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut writer = stdout;
        let reader = BufReader::new(stdin);
        let mut lines = reader.lines();

        let wallet = WalletMcpServer::new(rpc_url, private_key);
        while let Some(line) = lines.next_line().await? {
            if line.trim().is_empty() {
                continue;
            }

            let req: Request = match serde_json::from_str(&line) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Invalid JSON: {}", e);
                    continue;
                }
            };

            let result = match req.method.as_str() {
                "ping" => serde_json::json!({"message": "pong"}),
                "get_balance" => {
                    match serde_json::from_value::<GetBalanceRequest>(
                        req.params.clone().unwrap_or_default(),
                    ) {
                        Ok(params) => {
                            // 直接传给 wallet.swap_tokens
                            match wallet.get_balance(params).await {
                                Ok(res) => serde_json::json!({"result": res}),
                                Err(e) => serde_json::json!({"error": e.to_string()}),
                            }
                        }
                        Err(_) => serde_json::json!({"error": "invalid parameters"}),
                    }
                }
                "get_token_price" => {
                    match serde_json::from_value::<GetTokenPriceRequest>(
                        req.params.clone().unwrap_or_default(),
                    ) {
                        Ok(params) => {
                            // 直接传给 wallet.swap_tokens
                            match wallet.get_token_price(params).await {
                                Ok(res) => serde_json::json!({"result": res}),
                                Err(e) => serde_json::json!({"error": e.to_string()}),
                            }
                        }
                        Err(_) => serde_json::json!({"error": "invalid parameters"}),
                    }
                }
                // Handler
                "swap_tokens" => {
                    // 反序列化参数
                    match serde_json::from_value::<SwapTokensRequest>(
                        req.params.clone().unwrap_or_default(),
                    ) {
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
