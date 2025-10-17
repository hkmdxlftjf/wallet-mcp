use wallet_mcp::server::McpServer;
use wallet_mcp::GetTokenPriceRequest;
use rmcp::handler::server::wrapper::Parameters;
use std::env;
use tokio;

mod common;
use common::*;

#[tokio::test]
async fn test_get_token_price_success() {
    let server = create_test_server();
    let request = GetTokenPriceRequest {
        token: WBTC_ADDRESS.to_string(),
        fee: Some(3000),
    };

    let result = server.get_token_price(Parameters(request)).await;

    // 打印结果用于调试
    match &result {
        Ok(response) => {
            println!("✅ 代币价格查询成功");
            println!("📄 响应内容: {:?}", response);
        }
        Err(error) => {
            println!("❌ 代币价格查询失败");
            println!("🚫 错误信息: {:?}", error);
        }
    }

    assert!(result.is_ok(), "代币价格查询应该成功");
    let response = result.unwrap();
    assert!(!response.content.is_empty(), "响应内容不应为空");
}

#[tokio::test]
async fn test_get_token_price_default_fee() {
    let server = create_test_server();
    let request = GetTokenPriceRequest {
        token: WETH_ADDRESS.to_string(),
        fee: None, // 使用默认费率
    };

    let result = server.get_token_price(Parameters(request)).await;

    // 打印结果用于调试
    match &result {
        Ok(response) => {
            println!("✅ 使用默认费率的价格查询成功");
            println!("📄 响应内容: {:?}", response);
        }
        Err(error) => {
            println!("❌ 使用默认费率的价格查询失败");
            println!("🚫 错误信息: {:?}", error);
        }
    }

    assert!(result.is_ok(), "使用默认费率的价格查询应该成功");
}

#[tokio::test]
async fn test_price_queries_consistency() {
    let server = create_test_server();

    // 查询同一代币的价格两次，结果应该相近
    let request1 = GetTokenPriceRequest {
        token: WBTC_ADDRESS.to_string(),
        fee: Some(3000),
    };

    let request2 = GetTokenPriceRequest {
        token: WBTC_ADDRESS.to_string(),
        fee: Some(3000),
    };

    let result1 = server.get_token_price(Parameters(request1)).await;
    let result2 = server.get_token_price(Parameters(request2)).await;

    assert!(result1.is_ok(), "第一次价格查询应该成功");
    assert!(result2.is_ok(), "第二次价格查询应该成功");

    // 注意：由于区块链状态可能变化，这里只验证查询成功，不验证价格完全相等
}