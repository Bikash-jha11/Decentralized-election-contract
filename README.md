Decentralized Election Contract (Rust/Anchor)

A transparent, tamper-proof voting system built with Rust. This contract leverages the "security by design" philosophy of Rust to ensure that votes are cast uniquely, counted accurately, and stored immutably.

## Core Functionality

Proposal Creation: Authorized administrators can initialize an election with specific candidates.

One-Vote-Per-User: Uses PDA (Program Derived Address) mapping to ensure each public key can only vote once.

Real-time Results: Tallying is updated on-chain as soon as a vote transaction is confirmed.

State Locking: Ability to open and close the election at specific timestamps.

## Program Architecture
Unlike Solidity’s "Contract" model, this Rust program follows a Stateless Logic model where data is stored in separate accounts.

## Technical Specifications
Language: Rust 1.70+

Framework: Anchor (for Solana) or ink! (for Polkadot)

Safety: Utilizes Rust’s strict type system to prevent integer overflows and unauthorized access.

Storage: Efficient account serialization using Borsh.

