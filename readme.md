# Bundler Guide

## Installation

### Install Protocol Buffers (protobuf)

To install Protocol Buffers, follow the instructions based on your operating system:

#### macOS

```bash
brew install protobuf
```

#### Ubuntu/Debian

```bash
sudo apt update
sudo apt install -y protobuf-compiler
```

#### Windows

Download the latest release of Protocol Buffers from the [official GitHub releases page](https://github.com/protocolbuffers/protobuf/releases). Extract the files and add the `bin` directory to your system's PATH.

### Install Rust

If you haven't installed Rust yet, you can do so using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Setup the Project

Clone the repository and navigate into the project directory:

```bash
git clone https://github.com/your-username/your-rust-project.git
cd your-rust-project
```

## Cargo Commands

To build the project:

```bash
cargo build
```

To run the project:

```bash
cargo run
```

To run tests:

```bash
cargo test
```

## Mode

The project allows you to select between different modes of operation. As you go through the mode selection, the bot will automatically ask for all the required settings.

Available modes:

- **Generate Wallets:** Generate new wallets.
- **Create LUT:** Create Lookup Tables (LUT).
- **Wrap SOL & ATAs:** Wrap SOL and Associated Token Accounts (ATAs).
- **Bundle Liquidity:** Bundle liquidity into pools.

## Settings

The settings for the project are stored in a configuration file or environment variables. Here is an example configuration in JSON format:

```json
{
  "RPC-URL": "https://api.mainnet-beta.solana.com",
  "BLOCK-ENGINE-URL": "https://ny.mainnet.block-engine.jito.wtf",
  "TOKEN-MINT": "",
  "MARKET-ADDRESS": "",
  "POOL-ID": "",
  "DEPLOYER-PRIVATE-KEY": "",
  "BUYER-PRIVATE-KEY": "",
  "LUT-KEY": "",
  "VOLUME-LUT-KEY": ""
}
```

Ensure to update the `settings.json` file with your specific values. The bot will guide you through the process of entering all required settings as you select each mode. Keep your private keys and sensitive data secure.
