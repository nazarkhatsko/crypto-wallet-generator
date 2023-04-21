# Crypto Wallet Generator

For build
```bash
cargo build
```

For formating code
```bash
cargo fmt
```

Commands
- `blockchain` - enum with Bitcoin, Ethereum (_required_)
- `mnemonic` - generate keys based on mnemonic (_option_)
- `file` - generate keys based on file (_option_)
- `output` - file name with wallet in JSON format (_option_)

Example
```bash
./target/debug/crypto-wallet-generator --blockchain bitcoin
```
