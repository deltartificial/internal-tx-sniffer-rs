## internal-tx-sniffer-rs

Trace and analyze internal EVM transactions in Rust using alloy. A powerful tool for blockchain developers to inspect transaction execution, focusing on CREATE2/CREATE3 deployments and ERC20 token detection.

### Features

- Transaction tracing with detailed execution steps
- CREATE2 deployment detection
- CREATE3 deployment detection
- Automatic ERC20 token analysis for deployed contracts
  - Token name detection
  - Symbol detection
  - Decimals detection
  - Total supply information
- Clean output formatting
- Built with Rust for optimal performance
- Detailed call trace output saved to file

### Usage

#### Basic Transaction Tracing

```bash
cargo run --release -- --rpc-url YOUR_RPC_URL --hash TRANSACTION_HASH
```

#### Search for CREATE2 Deployments

```bash
cargo run --release -- \
    --rpc-url YOUR_RPC_URL \
    --hash TRANSACTION_HASH \
    --search create2
```

#### Search for CREATE3 Deployments

```bash
cargo run --release -- \
    --rpc-url YOUR_RPC_URL \
    --hash TRANSACTION_HASH \
    --search create3
```

#### Example

```bash
cargo run --release -- \
    --rpc-url https://nd-418-459-126.p2pify.com/8763cb5a211e1d4345acd51bde484c00/ext/bc/C/rpc \
    --hash 0xb7a88c16d7b8b06b9cd4a9666a9c9a5cd0cb89aba050f6bb383e0740531c8fa0 \
    --search create2
```

### Output

The tool provides:
- A detailed call trace saved to `out/call_trace.txt`
- For CREATE2/CREATE3 deployments:
  - Deployment address
  - Deployer address
  - Transaction value
  - ERC20 token information (if the deployed contract is an ERC20 token):
    - Token name
    - Token symbol
    - Decimals
    - Total supply
