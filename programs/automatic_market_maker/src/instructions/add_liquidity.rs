use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
#[derive(Accounts)]
pub struct AddLiquidity<'a> {
    pub user: Signer<'a>,

    pub user_token_a_mint_ata: InterfaceAccount<'a, TokenAccount>,
    pub user_token_b_mint_ata: InterfaceAccount<'a, TokenAccount>,

    pub vaut_token_a_ata: InterfaceAccount<'a, TokenAccount>,
    pub vaut_token_b_ata: InterfaceAccount<'a, TokenAccount>,
    pub vaut_token_lp_mint_ata: InterfaceAccount<'a, TokenAccount>,

    pub pool_state_account: Account<'a, PoolState>,
    pub system_program: Program<'a, System>,
    pub token_program: Interface<'a, TokenInterface>,
}
pub fn handler(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
    Ok(())
}
