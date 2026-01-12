# Solana AMM

A constant product (x*y=k) automated market maker built on Solana with Anchor.

## Features

- **Token Swaps**: Bi-directional swaps with slippage protection
- **Liquidity Provision**: Add/remove liquidity, earn LP tokens
- **Secure**: PDA-based vaults, comprehensive account validation

## Architecture

### Instructions

**`initialize_pool`** - Creates new liquidity pool for token pair  
**`add_liquidity`** - Deposits tokens, mints LP tokens  
- First deposit: `sqrt(amount_a * amount_b)`
- After: Proportional to reserves

**`remove_liquidity`** - Burns LP tokens, withdraws proportionally  
**`swap`** - Trades tokens using `(amount_in * reserve_out) / (reserve_in + amount_in)`

### Account Structure
```rust
pub struct PoolState {
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub lp_token_mint: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub authority: Pubkey,
    pub bump: u8,
}
```

## Quick Start
```bash
# Build
anchor build

# Test
anchor test

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

## Example Usage
```typescript
// Initialize pool
await program.methods.initializePool(tokenAMint, tokenBMint).rpc();

// Add liquidity
await program.methods.addLiquidity(new BN(1000), new BN(2000)).rpc();

// Swap A → B
await program.methods.swap(new BN(100), new BN(95), true).rpc();

// Remove liquidity
await program.methods.removeLiquidity(new BN(500)).rpc();
```

## Tech Stack

- Anchor 0.30.x
- Rust 1.75+
- Solana Web3.js


