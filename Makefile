# Wallet MCP Makefile
# 提供项目构建、测试、开发环境管理等功能

# 默认配置
FORK_URL ?= https://api.zan.top/node/v1/eth/mainnet/7e171469dd60477baf95f990a25e6562
ANVIL_PORT ?= 8545
ANVIL_HOST ?= 0.0.0.0
ANVIL_ACCOUNTS ?= 1
STATE_FILE ?= state.json
RUST_LOG ?= info

# 颜色定义
RED = \033[0;31m
GREEN = \033[0;32m
YELLOW = \033[0;33m
BLUE = \033[0;34m
NC = \033[0m # No Color

# 默认目标
.DEFAULT_GOAL := help

# 帮助信息
.PHONY: help
help: ## 显示帮助信息
	@echo "$(BLUE)Wallet MCP 项目 Makefile$(NC)"
	@echo ""
	@echo "$(YELLOW)可用命令:$(NC)"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  $(GREEN)%-20s$(NC) %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@echo ""
	@echo "$(YELLOW)环境变量:$(NC)"
	@echo "  $(GREEN)FORK_URL$(NC)        - 以太坊分叉URL (默认: $(FORK_URL))"
	@echo "  $(GREEN)ANVIL_PORT$(NC)      - Anvil端口 (默认: $(ANVIL_PORT))"
	@echo "  $(GREEN)ANVIL_HOST$(NC)      - Anvil主机 (默认: $(ANVIL_HOST))"
	@echo "  $(GREEN)ANVIL_ACCOUNTS$(NC)  - Anvil账户数量 (默认: $(ANVIL_ACCOUNTS))"
	@echo "  $(GREEN)STATE_FILE$(NC)      - 状态文件路径 (默认: $(STATE_FILE))"
	@echo "  $(GREEN)RUST_LOG$(NC)        - 日志级别 (默认: $(RUST_LOG))"

# 构建相关命令
.PHONY: build
build: ## 构建项目
	@echo "$(BLUE)构建项目...$(NC)"
	cargo build

.PHONY: build-release
build-release: ## 构建发布版本
	@echo "$(BLUE)构建发布版本...$(NC)"
	cargo build --release

.PHONY: clean
clean: ## 清理构建文件
	@echo "$(BLUE)清理构建文件...$(NC)"
	cargo clean
	@if [ -f "$(STATE_FILE)" ]; then \
		echo "$(YELLOW)删除状态文件: $(STATE_FILE)$(NC)"; \
		rm -f "$(STATE_FILE)"; \
	fi

# 测试相关命令
.PHONY: test
test: ## 运行所有测试
	@echo "$(BLUE)运行测试套件...$(NC)"
	RUST_LOG=$(RUST_LOG) cargo test

# Anvil 节点管理
.PHONY: anvil-start
anvil-start: ## 启动Anvil本地节点
	@echo "$(BLUE)启动Anvil节点...$(NC)"
	@echo "$(YELLOW)配置:$(NC)"
	@echo "  Fork URL: $(FORK_URL)"
	@echo "  Port: $(ANVIL_PORT)"
	@echo "  Host: $(ANVIL_HOST)"
	@echo "  Accounts: $(ANVIL_ACCOUNTS)"
	@echo "  State File: $(STATE_FILE)"
	@if command -v anvil >/dev/null 2>&1; then \
		anvil --fork-url $(FORK_URL) --port $(ANVIL_PORT) --host $(ANVIL_HOST) --accounts $(ANVIL_ACCOUNTS) --dump-state $(STATE_FILE) --load-state $(STATE_FILE); \
	else \
		echo "$(RED)错误: anvil 未安装。请先安装 Foundry。$(NC)"; \
		echo "$(YELLOW)安装命令: curl -L https://foundry.paradigm.xyz | bash && foundryup$(NC)"; \
		exit 1; \
	fi

.PHONY: anvil-stop
anvil-stop: ## 停止Anvil节点
	@echo "$(BLUE)停止Anvil节点...$(NC)"
	@pkill -f "anvil.*--port $(ANVIL_PORT)" || echo "$(YELLOW)没有找到运行中的Anvil节点$(NC)"

.PHONY: anvil-restart
anvil-restart: anvil-stop anvil-start ## 重启Anvil节点


.PHONY: dev-run
dev-run: ## 运行开发服务器
	@echo "$(BLUE)启动MCP服务器...$(NC)"
	RUST_LOG=$(RUST_LOG) cargo run --bin wallet-mcp

.PHONY: dev-run-with-anvil
dev-run-with-anvil: ## 启动Anvil节点并运行MCP服务器
	@echo "$(BLUE)启动完整开发环境...$(NC)"
	@make anvil-start &
	@sleep 5
	@echo "$(BLUE)启动MCP服务器...$(NC)"
	ETH_RPC_URL=http://$(ANVIL_HOST):$(ANVIL_PORT) RUST_LOG=$(RUST_LOG) cargo run --bin wallet-mcp

