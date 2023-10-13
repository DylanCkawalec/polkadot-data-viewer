# Polkadot Data Viewer

## Overview

The Polkadot Data Viewer is a Rust application designed to interact with the Polkadot blockchain. It fetches and displays various pieces of information such as the finalized block hash, block header, extrinsics, and chain name. The application is structured in a modular fashion, with separate directories managing different aspects of the project flow.

## Directory Structure

### `src/collect`

This directory is responsible for collecting and storing the fetched data temporarily. It contains mechanisms to set and get data like the finalized block hash, block header, extrinsics, extrinsic hash, and chain name. The data stored here is used across different parts of the application.

### `src/utils`

This directory contains utility functions that handle the fetching and processing of data from the Polkadot blockchain. It includes functionalities such as sending HTTP requests, handling RPC calls, and formatting and printing the fetched data.

## Workflow

1. **Fetching the Finalized Block Hash:**
   - The application starts by fetching the hash of the finalized block from the Polkadot blockchain.
   - The hash is stored using functionalities provided in the `collect` directory.

2. **Fetching and Printing Block Header and Extrinsics:**
   - With the obtained block hash, the application fetches the block header and extrinsics.
   - The fetched data is displayed in a formatted manner, and the details are also stored for later use.

3. **Submitting an Extrinsic:**
   - An extrinsic (a piece of information representing an action in the blockchain) is submitted to the blockchain.
   - The hash of the submitted extrinsic is stored, and the result can be used for further interactions or validations.

4. **Fetching and Setting the Chain Name:**
   - The application fetches the chain name from the Polkadot blockchain and stores it for later use.

5. **Displaying the Collected Data:**
   - Finally, the application prints out all the collected data, such as the block hash, block header, extrinsics, and chain name, in a well-formatted and colored output.

## Usage

Run the application by executing the following command:

```bash
cargo build
cargo run
```

This will start the application, and it will perform the steps as per the workflow, displaying the fetched and processed data from the Polkadot blockchain.
