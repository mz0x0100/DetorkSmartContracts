# Detork Smart Contracts

This repository contains the smart contracts and supporting code for the DeTork escrow system, built using [Solana](https://solana.com/) and [Anchor](https://book.anchor-lang.com/).

[Detork Live Devnet](https://detork.vercel.app)

## Overview

The main contract in this project is an **escrow program** that enables secure payments between a client and a freelancer. Funds are held in a program-derived account (PDA) until released by the client.

## Features

- **Initialize Escrow:** Client creates an escrow specifying a freelancer and amount. Funds are transferred to a vault PDA.
- **Release Funds:** Client can release funds from the vault to the freelancer upon completion of work.


## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Node.js](https://nodejs.org/)
- [Yarn](https://yarnpkg.com/)
- [Anchor CLI](https://book.anchor-lang.com/getting_started/installation.html)

### Install Dependencies

```sh
yarn install
anchor build
anchor test
```