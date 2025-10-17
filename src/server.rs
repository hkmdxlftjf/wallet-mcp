use crate::models::{GetBalanceRequest, GetTokenPriceRequest, SwapTokensRequest};
use crate::services::WalletService;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{
    handler::server::tool::ToolRouter, model::*, tool, tool_handler, tool_router,
    ErrorData as McpError, ServerHandler,
};

#[derive(Clone)]
pub struct McpServer {
    wallet_service: WalletService,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl McpServer {
    #[allow(dead_code)]
    pub fn new(private_key: String, rpc_url: String) -> Self {
        Self {
            tool_router: Self::tool_router(),
            wallet_service: WalletService::new(private_key, rpc_url),
        }
    }
    
    #[tool(description = "Query ETH and ERC20 token balances")]
    pub async fn get_balance(
        &self,
        request: Parameters<GetBalanceRequest>,
    ) -> Result<CallToolResult, McpError> {
        self.wallet_service.get_balance(request).await
    }

    #[tool(description = "Get current token price in USD or ETH")]
    pub async fn get_token_price(
        &self,
        request: Parameters<GetTokenPriceRequest>,
    ) -> Result<CallToolResult, McpError> {
        self.wallet_service.get_token_price(request).await
    }

    #[tool(description = "Execute a token swap on Uniswap V2 or V3")]
    pub async fn swap_tokens(
        &self,
        request: Parameters<SwapTokensRequest>,
    ) -> Result<CallToolResult, McpError> {
        self.wallet_service.swap_tokens(request).await
    }
}

#[tool_handler]
impl ServerHandler for McpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "This server provides wallet tools. Tools: get_balance get_token_price swap_tokens.".to_string(),
            ),
        }
    }
}
