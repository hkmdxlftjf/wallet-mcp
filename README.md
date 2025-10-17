
## 使用说明

### 前提条件

- rust
- anvil

### MCP 调用示例

1. get_balance请求
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "get_balance",
    "arguments": {
      "wallet_address": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
      "token_address": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
    }
  }
}
```
get_balance响应
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "1000.0 USDC"
      }
    ],
    "isError": false
  }
}
```

2. get_token_price 请求
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "get_token_price",
    "arguments": {
      "token": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
      "fee": 3000
    }
  }
}
```
get_token_price 请求
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "4036.350217 USDT"
      }
    ],
    "isError": false
  }
}
```

3. swap_tokens 请求
```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "method": "tools/call",
  "params": {
    "name": "swap_tokens",
    "arguments": {
      "from_token": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
      "to_token": "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599",
      "amount": 1.0,
      "slippage_pct": 50,
      "fee": 3000
    }
  }
}
```
swap_tokens 响应
```json
{
  "jsonrpc": "2.0",
  "id": 4,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "estimated_out: 0.03628441 WBTC | gas_price: 3628441 Gwei"
      }
    ],
    "isError": false
  }
}
```

### MCP 配置

```
{
  "mcpServers": {
    "counter": {
      "command": "PATH-TO/wallet-mcp/target/release/wallet-mcp",
      "args": [],
      "env": {
        "ETH_RPC_URL": "http://localhost:8545",
        "PRIVATE_KEY": "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
      }
    }
  }
}
```

- ETH_RPC_URL 为以太坊节点地址
- PRIVATE_KEY 为私钥地址

### 本地 anvil

启动

```
make anvil-start
```

停止

```
make anvil-stop
```



# Programming Assignment: Ethereum Trading MCP Server

## Overview

Build a Model Context Protocol (MCP) server in Rust that enables AI agents to query balances and execute token swaps on Ethereum.

## Requirements

### Core Functionality

Implement an MCP server with the following tools:

1. **`get_balance`** - Query ETH and ERC20 token balances
   - Input: wallet address, optional token contract address
   - Output: balance information with proper decimals
2. **`get_token_price`** - Get current token price in USD or ETH
   - Input: token address or symbol
   - Output: price data
3. **`swap_tokens`** - Execute a token swap on Uniswap V2 or V3
   - Input: from_token, to_token, amount, slippage tolerance
   - Output: simulation result showing estimated output and gas costs
   - **Important**: Construct a real Uniswap transaction and submit it to the blockchain for simulation (using `eth_call` or similar). The transaction should NOT be executed on-chain.

### Technical Stack

**Required:**

- Rust with async runtime (tokio)
- Ethereum RPC client library (ethers-rs or alloy)
- MCP SDK for Rust ([rmcp](https://github.com/modelcontextprotocol/rust-sdk)) or implement JSON-RPC 2.0 manually
- Structured logging (tracing)

### Constraints

- Must connect to real Ethereum RPC (use public endpoints or Infura/Alchemy)
- Balance queries must fetch real on-chain data
- For swaps: construct real Uniswap V2/V3 swap transactions and simulate them using RPC methods
- Transaction signing: implement basic wallet management (e.g., private key via environment variable or config file)
- Use `rust_decimal` or similar for financial precision

## Deliverables

1. **Working code** - Rust project that compiles and runs
2. **README** with:
    - Setup instructions (dependencies, env vars, how to run)
    - Example MCP tool call (show JSON request/response)
    - Design decisions (3-5 sentences on your approach)
    - Known limitations or assumptions

## 🧪 测试套件

本项目包含全面的集成测试套件，覆盖所有核心功能模块。测试套件位于 `tests/` 目录中，包含以下测试类别：

### 测试覆盖范围

1. **余额查询测试**
   - ETH 余额查询（正常情况）
   - ERC20 代币余额查询
   - 无效地址处理
   - 零地址边界条件

2. **代币价格查询测试**
   - 标准代币价格查询
   - 默认费率处理
   - 无效代币地址处理
   - 价格查询一致性验证

3. **代币交换模拟测试**
   - 正常交换模拟
   - 高滑点处理
   - 零数量边界条件
   - 相同代币交换错误处理
   - 无效代币地址处理

4. **并发和性能测试**
   - 多个并发请求处理
   - 服务器信息验证
   - 边界条件测试

### 运行测试

使用 Makefile 提供的便捷命令：

```bash
# 运行所有测试
make test

# 运行集成测试
make test-integration

# 运行单元测试
make test-unit

# 运行详细测试输出
make test-verbose

# 生成测试覆盖率报告
make test-coverage
```

### 测试环境配置

测试需要以下环境变量：
- `ETH_RPC_URL`: 以太坊RPC节点URL
- `RUST_LOG`: 日志级别（可选，默认为info）

测试使用的默认配置：
- 测试钱包地址: `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266` (Anvil默认账户)
- 测试私钥: Anvil默认私钥
- 测试代币: WETH, USDC, WBTC

## 🛠️ Makefile 使用指南

项目提供了功能完善的 Makefile，支持开发、测试、部署的完整工作流程。

### 主要命令分类

#### 构建命令
```bash
make build              # 构建项目
make build-release      # 构建发布版本
make clean              # 清理构建文件
```

#### 测试命令
```bash
make test               # 运行所有测试
make test-integration   # 运行集成测试
make test-unit          # 运行单元测试
make test-verbose       # 运行详细测试输出
make test-coverage      # 生成测试覆盖率报告
```

#### Anvil 节点管理
```bash
make anvil-start        # 启动Anvil本地节点
make anvil-stop         # 停止Anvil节点
make anvil-restart      # 重启Anvil节点
make anvil-status       # 检查Anvil节点状态
```

#### 开发环境管理
```bash
make dev-setup          # 设置开发环境
make dev-run            # 运行开发服务器
make dev-run-with-anvil # 启动Anvil节点并运行MCP服务器
```

#### 代码质量检查
```bash
make fmt                # 格式化代码
make fmt-check          # 检查代码格式
make clippy             # 运行Clippy代码检查
make check              # 检查代码编译
make audit              # 安全审计
```

#### 示例运行
```bash
make example-local      # 运行本地测试示例
make example-swap       # 运行交换测试示例
make example-chainlink  # 运行Chainlink测试示例
```

### Anvil 节点配置

Makefile 支持通过环境变量自定义 Anvil 节点配置：

```bash
# 使用自定义配置启动 Anvil
FORK_URL=https://your-rpc-url.com \
ANVIL_PORT=8545 \
ANVIL_HOST=0.0.0.0 \
ANVIL_ACCOUNTS=10 \
make anvil-start
```

支持的环境变量：
- `FORK_URL`: 以太坊分叉URL（默认：https://api.zan.top/node/v1/eth/mainnet/7e171469dd60477baf95f990a25e6562）
- `ANVIL_PORT`: Anvil端口（默认：8545）
- `ANVIL_HOST`: Anvil主机（默认：0.0.0.0）
- `ANVIL_ACCOUNTS`: Anvil账户数量（默认：1）
- `STATE_FILE`: 状态文件路径（默认：state.json）

### 快速开始

```bash
# 一键设置开发环境
make quick-start

# 安装所需开发工具
make install-tools

# 运行完整CI检查
make ci

# 提交前检查
make pre-commit
```

### 获取帮助

```bash
make help               # 显示所有可用命令和说明
```

3. **Tests** - Demonstrate core functionality

## Development Approach

You're **encouraged** to use AI assistants (Cursor, Claude Code, GitHub Copilot, etc.) while working on this assignment. However, the solution should demonstrate your understanding of:

- Rust and async programming
- Ethereum fundamentals
- System design and architecture

The code will be reviewed for comprehension and design decisions.

## Submission

Create a GitHub repository and share the link. Ensure:

- `cargo build` compiles successfully
- `cargo test` passes
- README has clear setup instructions
- Code is well-organized and readable