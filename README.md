# SOLBOX Anchor

## Prerequisites

Before you begin, ensure you have installed:

- [Rust](https://www.rust-lang.org/)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html)
- [Node.js](https://nodejs.org/)

## Building the Project

Navigate to the project directory and run:

```bash
cargo build-bpf
```

## Deployment

### 1. Set up Solana Cluster

For Devnet:
```bash
solana config set --url https://api.devnet.solana.com
```

For Mainnet:
```bash
solana config set --url https://api.mainnet-beta.solana.com
```

### 2. Deploy the Contract

Deploy to your configured Solana cluster:
```bash
anchor deploy
```

## Testing the Program

Run the test suite:
```bash
anchor test
```

Ensure your Solana environment is properly configured for local testing. For Devnet or Mainnet testing, verify you have a funded wallet and valid network connection.
