use wallet_mcp::server::McpServer;
use std::env;

// 测试常量
pub const TEST_WALLET_ADDRESS: &str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"; // Anvil默认账户
pub const TEST_PRIVATE_KEY: &str = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"; // Anvil默认私钥
pub const WETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
pub const USDC_ADDRESS: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
pub const WBTC_ADDRESS: &str = "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599";

/// 获取测试用的RPC URL
pub fn get_test_rpc_url() -> String {
    env::var("ETH_RPC_URL").unwrap_or_else(|_| "http://localhost:8545".to_string())
}

/// 创建测试服务器实例
pub fn create_test_server() -> McpServer {
    let rpc_url = get_test_rpc_url();
    McpServer::new(TEST_PRIVATE_KEY.to_string(), rpc_url)
}