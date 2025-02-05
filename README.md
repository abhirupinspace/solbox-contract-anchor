# SOLBOX SMart Contract Test Anchor

## Prerequisites

Before you begin, make sure you have the following installed on your machine:

- [Rust](https://www.rust-lang.org/): The programming language used for smart contract development.
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools): Command-line tools for Solana.
- [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html): Framework for Solana smart contract development.
- [Node.js](https://nodejs.org/): For running the frontend (if applicable).

## Getting Started

### 1. Clone the repository
Clone the repository to your local machine:

```bash
git clone https://github.com/abhirupinspace/SOLBOX-Anchor.git
cd SOLBOX-Anchor
2. Install dependencies
To install the required dependencies for Anchor:

bash
Copy
Edit
cargo build-bpf
Make sure you have the required dependencies installed before building, such as Solana toolchain and Rust.

3. Set up Solana Cluster
Set up your Solana environment to point to the appropriate cluster:

bash
Copy
Edit
solana config set --url https://api.devnet.solana.com
For production, you can switch to the mainnet:

bash
Copy
Edit
solana config set --url https://api.mainnet-beta.solana.com
4. Deploy the Contract
To deploy the contract to the Solana network:

bash
Copy
Edit
anchor deploy
5. Interact with the Smart Contract
After deployment, interact with the smart contract using Solana’s CLI or any other method you prefer, such as a frontend dApp.

Example:

bash
Copy
Edit
solana transfer <DESTINATION_ADDRESS> 1 --from <KEYPAIR_PATH>
Development
To run tests locally and interact with the program, you can use the following command:

bash
Copy
Edit
anchor test
Make sure your Solana environment is correctly set up for the local network.