
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