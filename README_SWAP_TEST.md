# Uniswap V3 代币交换模拟测试

这个示例展示了如何使用 `swap_tokens` 方法在 Uniswap V3 上进行代币交换的模拟测试。

## 功能特性

- 🔄 **真实交易模拟**: 使用 `eth_call` 进行无风险的交易模拟
- 💰 **滑点保护**: 自动计算最小输出金额，防止价格滑点损失
- ⛽ **Gas 估算**: 准确估算交易所需的 gas 费用
- 📊 **详细输出**: 显示输入、预估输出、实际输出和 gas 成本
- 🛡️ **错误处理**: 完善的错误处理和友好的错误信息

## 运行步骤

1. **设置环境变量**:
   ```bash
   export ETH_RPC_URL="your_ethereum_rpc_url"
   ```

2. **运行测试**:
   ```bash
   cargo run --example swap_test
   ```

## 测试用例

### 1. ETH -> USDT
- 输入: 1.0 ETH
- 滑点: 0.5%
- 测试主流交易对的流动性

### 2. USDT -> WBTC
- 输入: 1000.0 USDT
- 滑点: 1.0%
- 测试稳定币到比特币的交换

### 3. WBTC -> UNI (高滑点)
- 输入: 0.1 WBTC
- 滑点: 2.0%
- 测试高滑点设置下的交换

### 4. 无效代币对
- 使用无效地址测试错误处理

## 输出示例

### 成功交换
```
🔄 Swap Simulation Results:
📥 Input: 1.0 WETH
📤 Estimated Output: 3456.78 USDT
📤 Actual Output (Simulated): 3454.32 USDT
💰 Minimum Output (0.5% slippage): 3439.12 USDT
⛽ Estimated Gas: 185000 units
💸 Estimated Gas Cost: 0.0037 ETH
✅ Simulation Status: SUCCESS
```

### 失败交换
```
❌ Swap Simulation Failed:
📥 Input: 1.0 INVALID
📤 Estimated Output: 0 USDT
💰 Minimum Output (0.5% slippage): 0 USDT
🚫 Error: Contract call reverted
💡 Possible reasons: Insufficient liquidity, invalid token pair, or slippage too low
```

## 技术实现

### 交易构建
- 使用 Uniswap V3 Router 合约
- 构建 `ExactInputSingleParams` 参数
- 设置 20 分钟的交易截止时间

### 模拟执行
- 使用 `eth_call` 进行无状态调用
- 不消耗 gas，不改变区块链状态
- 返回预期的交换结果

### Gas 估算
- 使用 `estimate_gas` 获取准确的 gas 消耗
- 获取当前网络的 gas price
- 计算总的交易成本

## 注意事项

1. **RPC 限制**: 免费的 RPC 节点可能有请求频率限制
2. **网络状态**: 模拟结果基于当前区块状态
3. **滑点设置**: 过低的滑点可能导致交易失败
4. **流动性**: 某些代币对可能流动性不足

## 自定义测试

你可以修改 `examples/swap_test.rs` 文件来测试其他代币对：

```rust
let swap_params = SwapParams {
    from_token: "代币A地址".to_string(),
    to_token: "代币B地址".to_string(),
    amount: "交换数量".to_string(),
    slippage: "滑点百分比".to_string(),
};
```

## 常见代币地址

- **WETH**: `0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2`
- **USDT**: `0xdAC17F958D2ee523a2206206994597C13D831ec7`
- **USDC**: `0xA0b86a33E6441b8C4505B8C4505B8C4505B8C4505`
- **WBTC**: `0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599`
- **UNI**: `0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984`