# Chainlink 价格查询测试示例

这个示例展示了如何使用 `wallet-mcp` 项目中的 Chainlink 价格查询功能。

## 功能说明

`get_token_price` 方法现在具有以下特性：

1. **优先查询 Chainlink**: 首先尝试从 Chainlink 的价格 feeds 获取代币价格
2. **自动回退机制**: 如果 Chainlink 没有对应的 feed，自动回退到 Uniswap V3 池查询
3. **错误处理**: 完善的错误处理机制，确保查询的稳定性

## 运行测试示例

### 1. 设置环境变量

在项目根目录创建 `.env` 文件：

```bash
ETH_RPC_URL=https://eth-mainnet.alchemyapi.io/v2/YOUR_API_KEY
```

或者使用其他以太坊 RPC 提供商：
- Infura: `https://mainnet.infura.io/v3/YOUR_PROJECT_ID`
- QuickNode: `https://YOUR_ENDPOINT.quiknode.pro/YOUR_TOKEN/`
- Alchemy: `https://eth-mainnet.alchemyapi.io/v2/YOUR_API_KEY`

### 2. 运行示例

```bash
cargo run --example chainlink_test
```

## 测试的代币

示例会测试以下代币的价格查询：

1. **ETH (WETH)** - `0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2`
   - 通常有 Chainlink ETH/USD feed
   
2. **WBTC** - `0x2260fac5e5542a773aa44fbcfedf7c193bc2c599`
   - 通常有 Chainlink BTC/USD feed
   
3. **USDC** - `0xa0b86a33e6441e6c7d3e4c5b4b6b4b6b4b6b4b6b`
   - 通常有 Chainlink USDC/USD feed
   
4. **UNI** - `0x1f9840a85d5af5bf1d1762f925bdaddc4201f984`
   - 可能没有 Chainlink feed，用于测试回退机制

## 预期输出

```
🔗 连接到以太坊主网: https://eth-mainnet.alchemyapi.io/v2/YOUR_API_KEY
📊 开始测试 Chainlink 价格查询...

🔍 查询 ETH 价格 (地址: 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2)...
✅ ETH 价格: 2345.67 USDT

🔍 查询 WBTC 价格 (地址: 0x2260fac5e5542a773aa44fbcfedf7c193bc2c599)...
✅ WBTC 价格: 43210.89 USDT

🔍 查询 USDC 价格 (地址: 0xa0b86a33e6441e6c7d3e4c5b4b6b4b6b4b6b4b6b)...
✅ USDC 价格: 1.00 USDT

🔍 查询 UNI 价格 (地址: 0x1f9840a85d5af5bf1d1762f925bdaddc4201f984)...
✅ UNI 价格: 12.34 USDT

🎉 测试完成！
```

## 技术细节

### Chainlink 价格查询流程

1. **构建 feed 地址**: 使用 `FeedRegistryInterface` 查询指定代币对 USDT 的 feed 地址
2. **获取价格数据**: 如果找到 feed，调用 `latestRoundData()` 获取最新价格
3. **价格转换**: 将 Chainlink 返回的价格数据转换为可读格式
4. **回退机制**: 如果 Chainlink 查询失败，自动使用 Uniswap V3 查询

### 错误处理

- 网络连接错误
- 合约调用失败
- 价格数据格式错误
- 代币地址无效

## 注意事项

1. **RPC 限制**: 确保你的 RPC 提供商有足够的请求配额
2. **网络延迟**: 示例中添加了 1 秒延迟避免请求过于频繁
3. **代币地址**: 确保使用正确的代币合约地址
4. **主网数据**: 这个示例连接到以太坊主网，使用真实的价格数据

## 自定义测试

你可以修改 `examples/chainlink_test.rs` 文件中的 `test_tokens` 数组来测试其他代币：

```rust
let test_tokens = vec![
    ("YOUR_TOKEN", "0xYOUR_TOKEN_ADDRESS"),
    // 添加更多代币...
];
```