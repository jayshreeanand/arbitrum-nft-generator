# Random Art NFT Generator - Stylus Edition

A generative art NFT project built on Arbitrum using Rust and Stylus. This project creates unique, on-chain SVG art for each minted NFT, with randomization based on block numbers.

## Overview

This project demonstrates the power of Stylus by implementing a fully on-chain NFT collection where each token's artwork is generated programmatically and stored entirely on the blockchain. The artwork consists of geometric shapes and connecting lines arranged in a grid pattern, with colors and rotations determined by block data.

Minted NFTs as viewed on metamask

<img width="380" alt="Screenshot 2025-02-26 at 10 40 59 AM" src="https://github.com/user-attachments/assets/d5d897f7-f551-4c4c-a69d-63c42bfe232c" />

### Image generation page
<img width="500" alt="Screenshot 2025-02-26 at 10 40 27 AM" src="https://github.com/user-attachments/assets/8d38c50b-2a79-49b4-bc95-1d3f7a26e740" />


## Features

- **Fully On-Chain Art**: All NFT artwork is generated and stored on-chain as SVG
- **Dynamic Generation**: Each NFT is unique, with artwork based on block numbers
- **ERC-721 Compatible**: Implements core ERC-721 functionality
- **Geometric Art**: Generates art using various shapes:
  - Circles
  - Rotated Rectangles
  - Triangles
  - Connected Lines

## Technical Details

### Smart Contract Features

- ERC-721 implementation
- On-chain SVG generation
- Base64 encoding for metadata and images
- Efficient storage using static buffers
- Block number-based randomization

### Art Generation

- 6x6 grid of geometric shapes
- Random color generation for each shape
- Shape rotation and positioning
- Connecting lines with opacity effects
- White background for better visibility

- 
### Screenshots

Interacting with contract on Remix IDE
<img width="1665" alt="Screenshot 2025-02-26 at 4 01 46 AM" src="https://github.com/user-attachments/assets/eb78c316-6b51-4da0-8f58-3773229561f4" />

Deployed Contract
<img width="278" alt="Screenshot 2025-02-26 at 3 30 39 AM" src="https://github.com/user-attachments/assets/4aca22f4-27d4-488b-814d-2ad64e1a5a9a" />

Transaction Request
<img width="357" alt="Screenshot 2025-02-26 at 3 30 24 AM" src="https://github.com/user-attachments/assets/4e74f008-46ce-4968-90c7-033d2292c42f" />

Successful NFT minting
<img width="1142" alt="Screenshot 2025-02-26 at 3 12 53 AM" src="https://github.com/user-attachments/assets/bb6dfeef-f77d-4e72-81f0-214e9be6c2e5" />

Stylus deploy on Nitro dev node
<img width="1339" alt="Screenshot 2025-02-26 at 2 58 53 AM" src="https://github.com/user-attachments/assets/5d322591-39c3-4f5c-af6b-3a100b99eeff" />




## Visual Elements

### Grid System

- 6x6 grid layout
- Each cell contains one shape
- Shapes can overlap edges

### Shapes

- Circles: Varying sizes and positions
- Rectangles: Different rotations and dimensions
- Triangles: Multiple orientations
- Lines: Connecting random points

### Colors

- Full RGB spectrum
- Opacity variations
- Color harmony through seed-based generation

### Depth

- Layered elements
- Opacity-based depth
- Overlapping shapes

## Technical Details

### Storage

- SVG data stored on-chain
- Minimal storage requirements
- No external dependencies

### Generation

- Deterministic based on token ID
- Block hash for additional randomness
- Gas-efficient calculations

### Viewing

- Real-time generation
- No external rendering required
- Standard SVG format

## Usage

1. Connect

## Getting Started

### Prerequisites

- Rust toolchain
- Cargo Stylus
- An Arbitrum testnet account with test ETH

### Installation

bash
Clone the repository
git clone [your-repo-url]
cd [your-repo-name]
Install dependencies
cargo install cargo-stylus
Build the project
cargo stylus check

### Deployment

Deploy to Arbitrum testnet
cargo stylus deploy

javascript
// Using ethers.js or web3.js
const tx = await contract.mint(recipientAddress);

### Viewing NFTs

The NFTs can be viewed on any NFT marketplace that supports Arbitrum and SVG rendering.

## Technical Architecture

### Storage

- Uses static buffers for efficient SVG and JSON generation
- SVG Buffer: 16KB
- JSON Buffer: 8KB

### Randomization

- Uses block numbers for deterministic randomization
- Converts block numbers to pseudo-random bytes
- Ensures reproducible results while maintaining uniqueness

### Art Generation Process

1. Creates a 6x6 grid
2. Generates random shapes for each grid cell
3. Adds connecting lines between random points
4. Applies random colors and rotations
5. Encodes as base64 SVG

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with Stylus SDK
- Deployed on Arbitrum

![Image](./header.png)

# Stylus Hello World

Project starter template for writing Arbitrum Stylus programs in Rust using the [stylus-sdk](https://github.com/OffchainLabs/stylus-sdk-rs). It includes a Rust implementation of a basic counter Ethereum smart contract:

```js
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Counter {
    uint256 public number;

    function setNumber(uint256 newNumber) public {
        number = newNumber;
    }

    function increment() public {
        number++;
    }
}
```

To set up more minimal example that still uses the Stylus SDK, use `cargo stylus new --minimal <YOUR_PROJECT_NAME>` under [OffchainLabs/cargo-stylus](https://github.com/OffchainLabs/cargo-stylus).

## Quick Start

Install [Rust](https://www.rust-lang.org/tools/install), and then install the Stylus CLI tool with Cargo

```bash
cargo install --force cargo-stylus cargo-stylus-check
```

Add the `wasm32-unknown-unknown` build target to your Rust compiler:

```
rustup target add wasm32-unknown-unknown
```

You should now have it available as a Cargo subcommand:

```bash
cargo stylus --help
```

Then, clone the template:

```
git clone https://github.com/OffchainLabs/stylus-hello-world && cd stylus-hello-world
```

### Testnet Information

All testnet information, including faucets and RPC endpoints can be found [here](https://docs.arbitrum.io/stylus/reference/testnet-information).

### ABI Export

You can export the Solidity ABI for your program by using the `cargo stylus` tool as follows:

```bash
cargo stylus export-abi
```

which outputs:

```js
/**
 * This file was automatically generated by Stylus and represents a Rust program.
 * For more information, please see [The Stylus SDK](https://github.com/OffchainLabs/stylus-sdk-rs).
 */

interface Counter {
    function setNumber(uint256 new_number) external;

    function increment() external;
}
```

Exporting ABIs uses a feature that is enabled by default in your Cargo.toml:

```toml
[features]
export-abi = ["stylus-sdk/export-abi"]
```

## Deploying

You can use the `cargo stylus` command to also deploy your program to the Stylus testnet. We can use the tool to first check
our program compiles to valid WASM for Stylus and will succeed a deployment onchain without transacting. By default, this will use the Stylus testnet public RPC endpoint. See here for [Stylus testnet information](https://docs.arbitrum.io/stylus/reference/testnet-information)

```bash
cargo stylus check
```

If successful, you should see:

```bash
Finished release [optimized] target(s) in 1.88s
Reading WASM file at stylus-hello-world/target/wasm32-unknown-unknown/release/stylus-hello-world.wasm
Compressed WASM size: 8.9 KB
Program succeeded Stylus onchain activation checks with Stylus version: 1
```

Next, we can estimate the gas costs to deploy and activate our program before we send our transaction. Check out the [cargo-stylus](https://github.com/OffchainLabs/cargo-stylus) README to see the different wallet options for this step:

```bash
cargo stylus deploy \
  --private-key-path=<PRIVKEY_FILE_PATH> \
  --estimate-gas
```

You will then see the estimated gas cost for deploying before transacting:

```bash
Deploying program to address e43a32b54e48c7ec0d3d9ed2d628783c23d65020
Estimated gas for deployment: 1874876
```

The above only estimates gas for the deployment tx by default. To estimate gas for activation, first deploy your program using `--mode=deploy-only`, and then run `cargo stylus deploy` with the `--estimate-gas` flag, `--mode=activate-only`, and specify `--activate-program-address`.

Here's how to deploy:

```bash
cargo stylus deploy \
  --private-key-path=<PRIVKEY_FILE_PATH>
```

The CLI will send 2 transactions to deploy and activate your program onchain.

```bash
Compressed WASM size: 8.9 KB
Deploying program to address 0x457b1ba688e9854bdbed2f473f7510c476a3da09
Estimated gas: 1973450
Submitting tx...
Confirmed tx 0x42db…7311, gas used 1973450
Activating program at address 0x457b1ba688e9854bdbed2f473f7510c476a3da09
Estimated gas: 14044638
Submitting tx...
Confirmed tx 0x0bdb…3307, gas used 14044638
```

Once both steps are successful, you can interact with your program as you would with any Ethereum smart contract.

## Calling Your Program

This template includes an example of how to call and transact with your program in Rust using [ethers-rs](https://github.com/gakonst/ethers-rs) under the `examples/counter.rs`. However, your programs are also Ethereum ABI equivalent if using the Stylus SDK. **They can be called and transacted with using any other Ethereum tooling.**

By using the program address from your deployment step above, and your wallet, you can attempt to call the counter program and increase its value in storage:

```rs
abigen!(
    Counter,
    r#"[
        function number() external view returns (uint256)
        function setNumber(uint256 number) external
        function increment() external
    ]"#
);
let counter = Counter::new(address, client);
let num = counter.number().call().await;
println!("Counter number value = {:?}", num);

let _ = counter.increment().send().await?.await?;
println!("Successfully incremented counter via a tx");

let num = counter.number().call().await;
println!("New counter number value = {:?}", num);
```

Before running, set the following env vars or place them in a `.env` file (see: [.env.example](./.env.example)) in this project:

```
RPC_URL=https://sepolia-rollup.arbitrum.io/rpc
STYLUS_CONTRACT_ADDRESS=<the onchain address of your deployed program>
PRIV_KEY_PATH=<the file path for your priv key to transact with>
```

Next, run:

```
cargo run --example counter --target=<YOUR_ARCHITECTURE>
```

Where you can find `YOUR_ARCHITECTURE` by running `rustc -vV | grep host`. For M1 Apple computers, for example, this is `aarch64-apple-darwin` and for most Linux x86 it is `x86_64-unknown-linux-gnu`

## Build Options

By default, the cargo stylus tool will build your project for WASM using sensible optimizations, but you can control how this gets compiled by seeing the full README for [cargo stylus](https://github.com/OffchainLabs/cargo-stylus). If you wish to optimize the size of your compiled WASM, see the different options available [here](https://github.com/OffchainLabs/cargo-stylus/blob/main/OPTIMIZING_BINARIES.md).

## Peeking Under the Hood

The [stylus-sdk](https://github.com/OffchainLabs/stylus-sdk-rs) contains many features for writing Stylus programs in Rust. It also provides helpful macros to make the experience for Solidity developers easier. These macros expand your code into pure Rust code that can then be compiled to WASM. If you want to see what the `stylus-hello-world` boilerplate expands into, you can use `cargo expand` to see the pure Rust code that will be deployed onchain.

First, run `cargo install cargo-expand` if you don't have the subcommand already, then:

```
cargo expand --all-features --release --target=<YOUR_ARCHITECTURE>
```

Where you can find `YOUR_ARCHITECTURE` by running `rustc -vV | grep host`. For M1 Apple computers, for example, this is `aarch64-apple-darwin`.

## License

This project is fully open source, including an Apache-2.0 or MIT license at your choosing under your own copyright.
