Decentralized Exchange (DEX) - Solidity
A peer-to-peer automated market maker (AMM) built on Ethereum. 
This protocol allows users to swap ERC-20 tokens, provide liquidity to earn fees, and maintain a trustless environment without a central intermediary.

## Core Features

Automated Market Making (AMM): Uses the Constant Product formula $x \cdot y = k$ to determine asset prices.

Liquidity Provision: Users can deposit token pairs into the pool to receive Liquidity Provider (LP) tokens.

Token Swaps: Instant execution of trades between supported ERC-20 pairs.

Fee Mechanism: A percentage fee (e.g., 0.3%) is collected on every trade and distributed proportionally to LP holders.

## Technical Architecture:
The system is designed with modularity and security in mind:
Factory Contract: Responsible for deploying and indexing individual exchange pairs.
Pair Contract: The core logic engine that holds the reserves, manages the $k$ value, and executes the swap, mint, and burn functions.
Router Contract:A wrapper that simplifies user interactions, handles multi-hop swaps, and manages "safety" checks like slippage tolerance.
