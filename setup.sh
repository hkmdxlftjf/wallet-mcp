#!/bin/bash
set -e

# =========================
# 配置
# =========================
RPC_URL="https://api.zan.top/node/v1/eth/mainnet/7e171469dd60477baf95f990a25e6562"
PORT=8545
HOST="0.0.0.0"

# 测试账户和私钥（anvil 默认第一个账户）
TEST_ACCOUNT="0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"

# ERC20 合约地址（Mainnet）
WETH="0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
USDT="0xdAC17F958D2ee523a2206206994597C13D831ec7"
WBTC="0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599"

# Mint 数量
ETH_AMOUNT="100"           # ETH
WETH_AMOUNT="1"            # WETH
USDT_AMOUNT="1000"         # USDT (6 decimals)
WBTC_AMOUNT="1"            # WBTC (8 decimals)

# =========================
# 启动 Anvil fork
# =========================
anvil --fork-url $RPC_URL \
      --port $PORT \
      --host $HOST \
      --accounts 10 &

ANVIL_PID=$!
echo "Anvil fork started with PID $ANVIL_PID"
sleep 3

# =========================
# 设置账户 ETH 余额
# =========================
ETH_WEI=$(cast to-wei $ETH_AMOUNT ether)
cast rpc anvil_setBalance $TEST_ACCOUNT $ETH_WEI --rpc-url http://127.0.0.1:$PORT
echo "Set $ETH_AMOUNT ETH to $TEST_ACCOUNT"

# =========================
# Mint WETH (deposit ETH)
# =========================
WETH_WEI=$(cast to-wei $WETH_AMOUNT ether)
cast send $WETH deposit \
     --value $WETH_WEI \
     --rpc-url http://127.0.0.1:$PORT \
     --private-key $PRIVATE_KEY
echo "Minted $WETH_AMOUNT WETH"

# =========================
# Impersonate USDT/WBTC owner 并转账
# =========================
# USDT owner 地址
USDT_OWNER="0x28C6c06298d514Db089934071355E5743bf21d60"
cast rpc anvil_impersonateAccount $USDT_OWNER --rpc-url http://127.0.0.1:$PORT
USDT_RAW=$(cast to-wei $USDT_AMOUNT 6) # USDT 6 decimals
cast send $USDT transfer $TEST_ACCOUNT $USDT_RAW \
     --rpc-url http://127.0.0.1:$PORT \
     --private-key $USDT_OWNER
cast rpc anvil_stopImpersonatingAccount $USDT_OWNER --rpc-url http://127.0.0.1:$PORT
echo "Minted $USDT_AMOUNT USDT"

# WBTC owner 地址
WBTC_OWNER="0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2" # 假设 fork 上能 impersonate
WBTC_RAW=$(cast to-wei $WBTC_AMOUNT 8)
cast rpc anvil_impersonateAccount $WBTC_OWNER --rpc-url http://127.0.0.1:$PORT
cast send $WBTC transfer $TEST_ACCOUNT $WBTC_RAW \
     --rpc-url http://127.0.0.1:$PORT \
     --private-key $WBTC_OWNER
cast rpc anvil_stopImpersonatingAccount $WBTC_OWNER --rpc-url http://127.0.0.1:$PORT
echo "Minted $WBTC_AMOUNT WBTC"

# =========================
# 查询余额
# =========================
echo "=== Final Balances ==="
echo "ETH: $(cast balance $TEST_ACCOUNT --rpc-url http://127.0.0.1:$PORT | cast from-wei ether)"
echo "WETH: $(cast call $WETH "balanceOf(address)(uint256)" $TEST_ACCOUNT --rpc-url http://127.0.0.1:$PORT | cast from-wei ether)"
echo "USDT: $(cast call $USDT "balanceOf(address)(uint256)" $TEST_ACCOUNT --rpc-url http://127.0.0.1:$PORT | cast from-wei 6)"
echo "WBTC: $(cast call $WBTC "balanceOf(address)(uint256)" $TEST_ACCOUNT --rpc-url http://127.0.0.1:$PORT | cast from-wei 8)"
