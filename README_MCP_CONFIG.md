# MCP 配置指南

本项目是一个基于 Rust 的以太坊钱包 MCP (Model Context Protocol) 服务器，提供钱包余额查询、代币价格获取和 Uniswap V3 交换模拟功能。

## 🚀 快速开始

### 1. 环境准备

确保已安装：
- Rust (1.70+)
- Cargo

### 2. 配置 RPC 节点

在项目根目录创建 `.env` 文件：
```bash
ETH_RPC_URL=https://eth-mainnet.g.alchemy.com/v2/your-api-key
```

或者使用免费节点（有频率限制）：
```bash
ETH_RPC_URL=https://eth-mainnet.g.alchemy.com/v2/demo
```

### 3. MCP 配置

#### 方式一：Claude Desktop 配置

将 `claude_desktop_config.json` 的内容添加到 Claude Desktop 的配置文件中：

**macOS 位置：**
```
~/Library/Application Support/Claude/claude_desktop_config.json
```

**Windows 位置：**
```
%APPDATA%\Claude\claude_desktop_config.json
```

#### 方式二：通用 MCP 配置

使用 `mcp-config.json` 文件配置其他 MCP 客户端。

### 4. 启动服务

```bash
# 构建项目
cargo build --release

# 启动 MCP 服务器
cargo run --bin wallet-mcp
```

## 📋 可用功能

### 1. 钱包余额查询
```json
{
  "method": "get_balance",
  "params": {
    "address": "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6"
  }
}
```

### 2. 代币价格查询
```json
{
  "method": "get_token_price", 
  "params": {
    "quote": "0xdAC17F958D2ee523a2206206994597C13D831ec7"
  }
}
```

### 3. Uniswap V3 交换模拟
```json
{
  "method": "swap_tokens",
  "params": {
    "from_token": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
    "to_token": "0xdAC17F958D2ee523a2206206994597C13D831ec7", 
    "amount": "0.1",
    "slippage": "0.5"
  }
}
```

## 🔧 配置选项

### 环境变量

| 变量名 | 描述 | 默认值 | 必需 |
|--------|------|--------|------|
| `ETH_RPC_URL` | 以太坊 RPC 节点 URL | 无 | ✅ |
| `PORT` | 服务器端口 | 3000 | ❌ |

### RPC 节点推荐

1. **Alchemy** (推荐)
   - 免费版：`https://eth-mainnet.g.alchemy.com/v2/demo`
   - 付费版：`https://eth-mainnet.g.alchemy.com/v2/your-api-key`

2. **Infura**
   - `https://mainnet.infura.io/v3/your-project-id`

3. **Ankr** (需要 API Key)
   - `https://rpc.ankr.com/eth/your-api-key`

## 🧪 测试

### 运行示例测试
```bash
# 本地逻辑测试（不需要网络）
cargo run --example local_test

# 简化交换测试
ETH_RPC_URL="your-rpc-url" cargo run --example simple_swap_test

# 完整交换测试
ETH_RPC_URL="your-rpc-url" cargo run --example swap_test

# Chainlink 价格测试
ETH_RPC_URL="your-rpc-url" cargo run --example chainlink_test
```

### 验证 MCP 连接
```bash
# 检查服务器状态
curl -X POST http://localhost:3000 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc": "2.0", "method": "initialize", "id": 1}'
```

## 🛠️ 故障排除

### 常见问题

1. **STF 错误**
   - ✅ 已修复：使用 Quoter 进行安全估算

2. **频率限制 (429 错误)**
   - 解决方案：使用付费 RPC 节点或增加请求间隔

3. **编译错误**
   - 确保 Rust 版本 >= 1.70
   - 运行 `cargo clean && cargo build`

4. **连接超时**
   - 检查网络连接
   - 验证 RPC URL 有效性

### 日志调试

启用详细日志：
```bash
RUST_LOG=debug cargo run --bin wallet-mcp
```

## 📚 API 文档

详细的 API 文档请参考：
- [README.md](./README.md) - 项目概述
- [README_SWAP_TEST.md](./README_SWAP_TEST.md) - 交换功能详解
- [README_CHAINLINK_TEST.md](./README_CHAINLINK_TEST.md) - 价格查询详解

## 🔒 安全注意事项

1. **API Key 保护**
   - 不要在代码中硬编码 API Key
   - 使用环境变量或配置文件

2. **网络安全**
   - 仅在可信网络环境中运行
   - 考虑使用 HTTPS 代理

3. **资源限制**
   - 监控 RPC 调用频率
   - 设置合理的超时时间

## 📞 支持

如有问题，请查看：
1. 项目 Issues
2. 示例代码
3. 测试用例

---

**注意：** 本项目仅用于模拟和测试，不执行实际的代币交易。