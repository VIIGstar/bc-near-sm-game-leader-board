### Pre:
1. Install cargo for RUST
2. Install near-cli to interact with near account

### Test
```
cargo test -- --nocapture
```

### Build to file wasm to deploy
```
cargo build --target wasm32-unknown-unknown --release
```

### Deploy smart contract to near account
```
near deploy --wasmFile target/wasm32-unknown-unknown/release/rust_game_leader_board.wasm --accountId <ACCOUNT>.testnet
```

### Call function to validate deployed successfully
1. Init (constructor)
```
near call <ACCOUNT>.testnet new --accountId <ACCOUNT>.testnet
```

2. Call any function
```
near call <ACCOUNT>.testnet <FunctionName> <Args?> --accountId <ACCOUNT>.testnet
```