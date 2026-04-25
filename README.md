# STI PaperVault
A verifiable research paper library for STI students using Stellar.

## Problem
Students cannot easily access approved research papers, leading to weak thesis outputs.

## Solution
Approved papers are stored with on-chain verification using Soroban.

## Timeline
- Week 1: Smart contract
- Week 2: Web UI
- Week 3: Integration

## Stellar Features Used
- Soroban smart contracts

## Vision
Build a trusted academic knowledge system.

## Prerequisites
- Rust
- Soroban CLI

## Build
soroban contract build

## Test
cargo test

## Deploy
soroban contract deploy

## Example
soroban contract invoke --id <contract_id> --fn register_paper --arg paper1 --arg <address>

## License
MIT
