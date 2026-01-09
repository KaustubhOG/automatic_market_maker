use crate::state::*;
use crate::error::AmmErrors;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [
            b"pool_state_account", 
            token_a_mint.key().as_ref(), 
            token_b_mint.key().as_ref()
        ],
        bump
    )]
    pub pool_state_account: Account<'info, PoolState>,

    pub token_a_mint: InterfaceAccount<'info, Mint>,
    pub token_b_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [b"lp_token_mint", pool_state_account.key().as_ref()],
        bump
    )]
    pub lp_token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        token::mint = token_a_mint,
        token::authority = user
    )]
    pub user_token_a_ata: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        token::mint = token_b_mint,
        token::authority = user
    )]
    pub user_token_b_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = lp_token_mint,
        token::authority = user
    )]
    pub user_lp_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = vault_token_a.key() == pool_state_account.vault_a
    )]
    pub vault_token_a: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = vault_token_b.key() == pool_state_account.vault_b
    )]
    pub vault_token_b: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
pub fn handler(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {

    require!(amount_a>0, AmmErrors::INCORRECT);
    require!(amount_b>0, AmmErrors::INCORRECT);


    Ok(())
}
