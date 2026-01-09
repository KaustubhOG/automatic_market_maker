use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_a_mint: InterfaceAccount<'info, Mint>,
    pub token_b_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = authority,
        seeds = [b"lp_token_mint", pool_state_account.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = pool_state_account,
    )]
    pub lp_token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = authority,
        seeds = [b"token_a_vault", pool_state_account.key().as_ref()],
        bump,
        token::mint = token_a_mint,
        token::authority = pool_state_account,
    )]
    pub token_a_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        seeds = [b"token_b_vault", pool_state_account.key().as_ref()],
        bump,
        token::mint = token_b_mint,
        token::authority = pool_state_account,
    )]
    pub token_b_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        space = 8 + PoolState::INIT_SPACE,
        seeds = [b"pool_state_account", authority.key().as_ref()],
        bump
    )]
    pub pool_state_account: Account<'info, PoolState>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<InitializePool>, token_a: Pubkey, token_b: Pubkey) -> Result<()> {
    Ok(())
}
