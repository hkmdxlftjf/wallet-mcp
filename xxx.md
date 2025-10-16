```angular2html
cast send 0xE592427A0AEce92De3Edee1F18E0157C05861564 \
          "exactInputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))" \
          "(0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2,0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48,3000,0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266,1761711005,1000000000000000000,0,0)" \
          --value 1ether \
          --rpc-url http://127.0.0.1:8545 \
          --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
```

```angular2html
cast call 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48 \
      "balanceOf(address)(uint256)" 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 \
      --rpc-url http://127.0.0.1:8545
```

```angular2html
cast call 0x2260fac5e5542a773aa44fbcfedf7c193bc2c599 \
      "balanceOf(address)(uint256)" 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 \
      --rpc-url http://127.0.0.1:8545
```

```angular2html
cast call 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 \
      "balanceOf(address)(uint256)" 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 \
      --rpc-url http://127.0.0.1:8545
```

```angular2html
cast send 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2 "deposit()" \
          --value 100ether \
          --rpc-url http://127.0.0.1:8545 \
          --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
```

```angular2html
cast send 0x2260fac5e5542a773aa44fbcfedf7c193bc2c599 "deposit()" \
          --value 100ether \
          --rpc-url http://127.0.0.1:8545 \
          --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
```

```angular2html
# 1 ETH 精确输入，滑点 0.5 %，池子费层 0.3 %
export RPC=http://127.0.0.1:8545
export PK=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80   # anvil 首账户私钥

cast send 0xE592427A0AEce92De3Edee1F18E0157C05861564 \
  --rpc-url $RPC --private-key $PK --value 1ether \
  "exactInputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))" \
  "(0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2,\
    0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599,\
    3000,\
    0x0000000000000000000000000000000000000000,\
    $(date +%s),\
    1ether,\
    0,\
    0)"
```

```angular2html
cast send 0xE592427A0AEce92De3Edee1F18E0157C05861564 \
        --rpc-url $RPC --private-key $PK --value 1ether \
        "exactInputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))" \
        "(0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2,\
     0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599,\
     3000,\
     $ME,\
     1762617643,\
     1000000000000000000,\
     3600000,\
     0)"
```
