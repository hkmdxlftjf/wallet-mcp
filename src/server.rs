use crate::types::{GetBalanceRequest, GetTokenPriceRequest, SwapTokensRequest};
use crate::wallet::WalletMcpServer;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

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

    /// 监听 11111 端口
    pub async fn run(&self, rpc_url: String, private_key: String, port: u16) -> anyhow::Result<()> {
        let listener = TcpListener::bind(("0.0.0.0", port)).await?;
        eprintln!("MCP wallet-server listening on {port} ...");

        let wallet = WalletMcpServer::new(rpc_url, private_key);

        loop {
            let (socket, _) = listener.accept().await?;
            let wallet = wallet.clone();
            // 每连接一个任务
            tokio::spawn(async move {
                if let Err(e) = handle_client(socket, wallet).await {
                    eprintln!("client error: {e}");
                }
            });
        }
    }
}

async fn handle_client(socket: TcpStream, wallet: WalletMcpServer) -> anyhow::Result<()> {
    let (reader, mut writer) = socket.into_split();
    let mut lines = BufReader::new(reader).lines();

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
                    Ok(p) => match wallet.get_balance(p).await {
                        Ok(res) => serde_json::json!({"result": res}),
                        Err(e) => serde_json::json!({"error": e.to_string()}),
                    },
                    Err(_) => serde_json::json!({"error": "invalid parameters"}),
                }
            }
            "get_token_price" => {
                match serde_json::from_value::<GetTokenPriceRequest>(
                    req.params.clone().unwrap_or_default(),
                ) {
                    Ok(p) => match wallet.get_token_price(p).await {
                        Ok(res) => serde_json::json!({"result": res}),
                        Err(e) => serde_json::json!({"error": e.to_string()}),
                    },
                    Err(_) => serde_json::json!({"error": "invalid parameters"}),
                }
            }
            "swap_tokens" => {
                match serde_json::from_value::<SwapTokensRequest>(
                    req.params.clone().unwrap_or_default(),
                ) {
                    Ok(p) => match wallet.swap_tokens(p).await {
                        Ok(res) => serde_json::json!({"result": res}),
                        Err(e) => serde_json::json!({"error": e.to_string()}),
                    },
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
            let mut buf = serde_json::to_string(&resp)? + "\n";
            writer.write_all(buf.as_bytes()).await?;
            writer.flush().await?;
        }
    }
    Ok(())
}
