# ZK Whitelist
## Zero Knowledge Powered Address Whitelisting

# Introduction
The `ZK Whitelist` repository houses a Rust program designed to facilitate the creation of a whitelist control system on the Ethereum blockchain. This program leverages the power of `Zero-Knowledge` proofs, specifically `SNARKs` through the `ZoKrates` toolbox, to establish a robust, and efficient whitelisting mechanism. Unlike conventional methods that may use Merkle Trees, this setup achieves a constant proof size irrespective of the number of entries on the whitelist, which is particularly advantageous for handling very large datasets.

# Benefits
**Efficiency**: The proof size remains constant regardless of the number of entries in the whitelist, ensuring high performance even as the dataset scales.

**Transparency**: While the setup is private, the address-proofs, and the verifier can be made public, allowing for a transparent yet secure whitelisting process.

**Ease of Integration**: The Rust program processes an input file containing Ethereum addresses, and produces two essential outputs for the whitelisting process: `verifier.sol` and `addresses-proof.json`.

**Extendability**: New address can be added to whitelist, indefinitely without any on-chain changes, as long as the secure setup is preserved. 

# How To Use
> **DISCLAIMER**: This code and presentation is preliminary, **unaudited** and subject to revision. THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND.

## Prerequisites

Ensure you have the following prerequisites installed on your machine:

* [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
* [ZoKrates](https://zokrates.github.io/gettingstarted.html)

## Step-by-Step Guide
1. **Clone repo:**
```sh
git clone https://github.com/SpiralOutDotEu/zk-whitelist.git
```
2. **Input Preparation:**
* Prepare a file named `addresses.txt` with Ethereum addresses, each on a new line.
3. **Program Execution:** 
* Run the Rust program provided in the repository by executing the following command in the terminal:
```sh
cargo run
```
4. **Outputs:**

  Upon successful execution, the program will generate a series of files required for the secure setup. Among these files, `verifier.sol` and `addresses-proof.json` are the primary outputs that will be used for further interactions with the token contract on the Ethereum blockchain.

- `verifier.sol`: This file contains the Solidity code for the verifier contract, which can be deployed to the Ethereum blockchain.
- `addresses-proof.json`: This file  contains the proofs and inputs for each Ethereum address included in the `addresses.txt` input file.

> ### Security Note: 
> The program will also generate several other files as part of the secure setup process. It's crucial to handle these files with care as they are sensitive and should be kept private. Except for `verifier.sol` and `addresses-proof.json`, **all other generated files should NOT be shared publicly** and should be stored securely to ensure the integrity and security of your setup.

5. **Smart Contract Deployment:**

* Deploy the `verifier.sol` using your preferred Ethereum development environment (e.g., Remix, Hardhat, Truffle, Forge etc.).

* Utilize the provided example token contract to integrate the verifier and utilize the address proofs from `address-proof.json` for minting tokens.

 * The `verifier.sol` and the `address-proof.json` should be from the same run of the program. At every setup run, ZoKrates produces new `toxic waste` to `secure the setup` so in every run the two files should be used in pair.

6. **Example Usage:**

* In the provided [example](https://github.com/SpiralOutDotEu/zk-whitelist/tree/master/examples) token [contract](https://github.com/SpiralOutDotEu/zk-whitelist/blob/master/examples/zkToken.sol), the mint function accepts Zero-Knowledge proof and inputs to mint tokens to whitelisted addresses.

* The [mint](https://github.com/SpiralOutDotEu/zk-whitelist/blob/a037d53abb141166ac2659c0a5f90b778c99958f/examples/zkToken.sol#L25) function ensures that the Ethereum address invoking the function matches the Ethereum address in the Zero-Knowledge proof, and that tokens haven't been claimed by that address before.
* If the verification succeeds, tokens are minted to the Ethereum address, marking the address as claimed to prevent double-minting.

# Note
The core of the system is the Rust program which orchestrates the generation of ZoKrates setup files, computation of witnesses, generation of proofs, and the aggregation of these proofs and inputs into a structured JSON file. The example Solidity contract demonstrates how these proofs and inputs can be utilized on-chain to manage a token minting process restricted to whitelisted addresses.

The utilization of Zero-Knowledge proofs in this manner provides a robust and scalable mechanism for managing whitelists on the Ethereum blockchain, outperforming traditional Merkle Tree based approaches  in scenarios involving extremely large datasets or when there is a continuous influx of new entries.