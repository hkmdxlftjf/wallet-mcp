#!/bin/bash

# Anvil 测试环境设置脚本
# 用于创建测试账户并分配代币余额

echo "🔧 设置 Anvil 测试环境..."

# Anvil 默认账户信息
TEST_ACCOUNT="0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
TEST_PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"

# 代币合约地址
USDT_ADDRESS="0xdAC17F958D2ee523a2206206994597C13D831ec7"
WETH_ADDRESS="0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"

# RPC URL
RPC_URL="http://localhost:8545"

echo "📋 测试账户信息:"
echo "地址: $TEST_ACCOUNT"
echo "私钥: $TEST_PRIVATE_KEY"
echo ""

# 检查 ETH 余额
echo "💰 检查 ETH 余额..."
cast balance $TEST_ACCOUNT --rpc-url $RPC_URL

# 为测试账户分配 USDT 余额
echo ""
echo "🪙 为测试账户分配 USDT 余额..."

# 找到一个有大量 USDT 的地址 (Binance 热钱包)
USDT_WHALE="0x28C6c06298d514Db089934071355E5743bf21d60"

# 模拟从 whale 地址转账 USDT
echo "从 $USDT_WHALE 转账 100,000 USDT 到测试账户..."
cast rpc anvil_impersonateAccount $USDT_WHALE --rpc-url $RPC_URL

# 转账 USDT (USDT 有 6 位小数)
cast send $USDT_ADDRESS \
  "transfer(address,uint256)" \
  $TEST_ACCOUNT \
  100000000000 \
  --from $USDT_WHALE \
  --rpc-url $RPC_URL \
  --unlocked

# 停止模拟
cast rpc anvil_stopImpersonatingAccount $USDT_WHALE --rpc-url $RPC_URL

# 检查 USDT 余额
echo ""
echo "💰 检查 USDT 余额..."
cast call $USDT_ADDRESS \
  "balanceOf(address)(uint256)" \
  $TEST_ACCOUNT \
  --rpc-url $RPC_URL

# 为测试账户分配 WETH 余额
echo ""
echo "🌊 为测试账户分配 WETH 余额..."

# 找到一个有大量 WETH 的地址
WETH_WHALE="0x8EB8a3b98659Cce290402893d0123abb75E3ab28"

echo "从 $WETH_WHALE 转账 100 WETH 到测试账户..."
cast rpc anvil_impersonateAccount $WETH_WHALE --rpc-url $RPC_URL

# 转账 WETH (WETH 有 18 位小数)
cast send $WETH_ADDRESS \
  "transfer(address,uint256)" \
  $TEST_ACCOUNT \
  100000000000000000000 \
  --from $WETH_WHALE \
  --rpc-url $RPC_URL \
  --unlocked

# 停止模拟
cast rpc anvil_stopImpersonatingAccount $WETH_WHALE --rpc-url $RPC_URL

# 检查 WETH 余额
echo ""
echo "💰 检查 WETH 余额..."
cast call $WETH_ADDRESS \
  "balanceOf(address)(uint256)" \
  $TEST_ACCOUNT \
  --rpc-url $RPC_URL

echo ""
echo "✅ Anvil 测试环境设置完成！"
echo ""
echo "📋 测试账户摘要:"
echo "地址: $TEST_ACCOUNT"
echo "私钥: $TEST_PRIVATE_KEY"
echo "ETH 余额: 10000 ETH (Anvil 默认)"
echo "USDT 余额: 100000 USDT"
echo "WETH 余额: 100 WETH"
echo ""
echo "🚀 现在可以使用这个账户进行代币交换测试了！"