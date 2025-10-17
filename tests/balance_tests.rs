use rmcp::handler::server::wrapper::Parameters;
use tokio;
use wallet_mcp::GetBalanceRequest;

mod common;
use common::*;

#[tokio::test]
async fn test_get_eth_balance_success() {
    let server = create_test_server();
    let request = GetBalanceRequest {
        wallet_address: TEST_WALLET_ADDRESS.to_string(),
        token_address: None, // 查询ETH余额
    };

    let result = server.get_balance(Parameters(request)).await;

    // 打印结果用于调试
    match &result {
        Ok(response) => {
            println!("✅ ETH余额查询成功");
            println!("📄 响应内容: {:?}", response);
        }
        Err(error) => {
            println!("❌ ETH余额查询失败");
            println!("🚫 错误信息: {:?}", error);
        }
    }

    assert!(result.is_ok(), "ETH余额查询应该成功");
    let response = result.unwrap();
    assert!(!response.content.is_empty(), "响应内容不应为空");
}

#[tokio::test]
async fn test_multiple_balance_queries() {
    let server = create_test_server();

    // 并发查询多个余额
    let eth_request = GetBalanceRequest {
        wallet_address: TEST_WALLET_ADDRESS.to_string(),
        token_address: None,
    };

    let usdc_request = GetBalanceRequest {
        wallet_address: TEST_WALLET_ADDRESS.to_string(),
        token_address: Some(USDC_ADDRESS.to_string()),
    };

    let wbtc_request = GetBalanceRequest {
        wallet_address: TEST_WALLET_ADDRESS.to_string(),
        token_address: Some(WBTC_ADDRESS.to_string()),
    };

    let (eth_result, usdc_result, wbtc_result) = tokio::join!(
        server.get_balance(Parameters(eth_request)),
        server.get_balance(Parameters(usdc_request)),
        server.get_balance(Parameters(wbtc_request))
    );

    assert!(eth_result.is_ok(), "ETH余额查询应该成功");
    assert!(usdc_result.is_ok(), "USDC余额查询应该成功");
    assert!(wbtc_result.is_ok(), "WBTC余额查询应该成功");
}
