use crate::handlers::{BalanceHandler, PriceHandler, SwapHandler};
use crate::models::{GetBalanceRequest, GetTokenPriceRequest, SwapTokensRequest};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{model::CallToolResult, ErrorData as McpError};

/// 钱包服务，封装所有钱包相关的业务逻辑
#[derive(Clone)]
pub struct WalletService {
    private_key: String,
    rpc_url: String,
}

impl WalletService {
    /// 创建新的钱包服务实例
    pub fn new(private_key: String, rpc_url: String) -> Self {
        Self {
            private_key,
            rpc_url,
        }
    }

    /// 处理余额查询请求
    pub async fn get_balance(
        &self,
        request: Parameters<GetBalanceRequest>,
    ) -> Result<CallToolResult, McpError> {
        let handler = BalanceHandler::new(self.rpc_url.clone());
        handler.handle_get_balance(request).await
    }

    /// 处理代币价格查询请求
    pub async fn get_token_price(
        &self,
        request: Parameters<GetTokenPriceRequest>,
    ) -> Result<CallToolResult, McpError> {
        let handler = PriceHandler::new(self.rpc_url.clone());
        handler.handle_get_token_price(request).await
    }

    /// 处理代币交换请求
    pub async fn swap_tokens(
        &self,
        request: Parameters<SwapTokensRequest>,
    ) -> Result<CallToolResult, McpError> {
        let handler = SwapHandler::new(self.private_key.clone(), self.rpc_url.clone());
        handler.handle_swap_tokens(request).await
    }

    /// 获取 RPC URL（用于测试或其他需要）
    pub fn rpc_url(&self) -> &str {
        &self.rpc_url
    }

    /// 获取私钥（谨慎使用，仅在必要时）
    pub fn private_key(&self) -> &str {
        &self.private_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_service_creation() {
        let service = WalletService::new(
            "test_private_key".to_string(),
            "https://test.rpc.url".to_string(),
        );
        
        assert_eq!(service.rpc_url(), "https://test.rpc.url");
        assert_eq!(service.private_key(), "test_private_key");
    }

    #[test]
    fn test_wallet_service_clone() {
        let service = WalletService::new(
            "test_key".to_string(),
            "https://test.url".to_string(),
        );
        
        let cloned_service = service.clone();
        assert_eq!(service.rpc_url(), cloned_service.rpc_url());
        assert_eq!(service.private_key(), cloned_service.private_key());
    }
}