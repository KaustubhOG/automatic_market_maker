use crate::state::*;
use crate::helper::sqrt::sqrt;
use crate::error::AmmErrors;
use anchor_spl::token::{};
use std::cmp::min;
use anchor_lang::prelude::*;
use anchor_spl::{token::{}, token_interface::{Mint, TokenAccount, TokenInterface}};

#[derive(Accounts)]
pub struct Swap <'a>{
    #[account(mut)]

    pub user: Signer<'a>,
    #[account(
        seeds=[b"pool_state_account", token_a_mint.key().as_ref(),token_b_mint.key().as_ref()],
        bump
    )]
    pub pool_state_account: Account<'a, PoolState>,

    pub token_a_mint: InterfaceAccount<'a, Mint>,
    pub token_b_mint: InterfaceAccount<'a, Mint>,

    #[account(mut,
    token::mint=token_a_mint,
    token::authority=user
    )]
    pub token_a_mint_user_ata: InterfaceAccount<'a, TokenAccount>,
     #[account(mut,
    token::mint=token_b_mint,
    token::authority=user
    )]
    pub token_b_mint_user_ata: InterfaceAccount<'a, TokenAccount>,

    #[account(mut,
    constraint = token_a_mint_vault_ata.key() == pool_state_account.vault_a
    )]
    pub token_a_mint_vault_ata: InterfaceAccount<'a, TokenAccount>,
     #[account(mut,
      constraint = token_b_mint_vault_ata.key() == pool_state_account.vault_b
    )]
    pub token_b_mint_vault_ata: InterfaceAccount<'a, TokenAccount>,

    pub token_program: Interface<'a, TokenInterface>,
}

pub fn handler(
    ctx: Context<Swap>,
    amount_to_swap: u64,
    minimum_amoun_to_get_after_swap: u64,
    swap_a_to_b: bool 
) -> Result<()> {
    Ok(())
}
