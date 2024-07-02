# Token Wallet on ICP Blockchain

## Overview

This project is a simple token wallet implemented on the Internet Computer Protocol (ICP) blockchain using Rust.

## Features

- Send and receive IRCRC2 tokens.
- Display current token balance.
- Basic wallet security features.


<!--## Getting Started-->

### Prerequisites
- Node.js
- dfx
- rust

## Setup

1. Install DFINITY SDK: [Installation Guide](https://smartcontracts.org/docs/developers-guide/cli-reference/dfx-parent.html)
2. Clone the repository:
   ```sh
   git clone https://github.com/Ansh-Rajput/token_wallet
   cd token_wallet

3. Start the local ICP network
   ```bash
   dfx start

4. Deploy the canisters
   ```bash 
   dfx deploy
  
5. Run the unit tests on backend:
   ```bash
   cargo test
