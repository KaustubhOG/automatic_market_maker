use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]

pub struct PoolState {
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub lp_token_mint: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub authority: Pubkey,
} 