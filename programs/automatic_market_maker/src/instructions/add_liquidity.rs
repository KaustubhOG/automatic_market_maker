use crate::state::*;
use crate::helper::sqrt::sqrt;
use crate::error::AmmErrors;
use anchor_spl::token::{self, Transfer, MintTo, TransferChecked};
use std::cmp::min;
use anchor_lang::prelude::*;
use anchor_spl::{token::{}, token_interface::{Mint, TokenAccount, TokenInterface,transfer_checked}};
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

    let is_first_deposit = ctx.accounts.lp_token_mint.supply == 0;

let lp_tokens_to_mint = if is_first_deposit {
    // First deposit: geometric mean
  
    sqrt(amount_a*amount_b) 

} else {
    // Subsequent: proportional to reserves, take minimum
    let reserve_a = ctx.accounts.vault_token_a.amount;
    let reserve_b = ctx.accounts.vault_token_b.amount;
    let total_supply = ctx.accounts.lp_token_mint.supply;
    
    let lp_from_a = (amount_a * total_supply) / reserve_a;
    let lp_from_b = (amount_b * total_supply) / reserve_b;
    
    min(lp_from_a, lp_from_b)
};
//token a transfer
token::transfer_checked(CpiContext::new(ctx.accounts.token_program.to_account_info(),TransferChecked{
    from :ctx.accounts.user_token_a_ata.to_account_info(),
    mint:ctx.accounts.token_a_mint.to_account_info(),
    to:ctx.accounts.vault_token_a.to_account_info(),
    authority: ctx.accounts.user.to_account_info()

}), amount_a, ctx.accounts.token_a_mint.decimals)?;

//token b transfer
token::transfer_checked(CpiContext::new(ctx.accounts.token_program.to_account_info(),TransferChecked{
    from :ctx.accounts.user_token_b_ata.to_account_info(),
    mint:ctx.accounts.token_b_mint.to_account_info(),
    to:ctx.accounts.vault_token_b.to_account_info(),
    authority:ctx.accounts.user.to_account_info()
}), amount_b, ctx.accounts.token_b_mint.decimals)?;

let token_a_key = ctx.accounts.token_a_mint.key();
let token_b_key = ctx.accounts.token_b_mint.key();

let seeds = &[
    b"pool_state_account".as_ref(),
    token_a_key.as_ref(),
    token_b_key.as_ref(),
    &[ctx.accounts.pool_state_account.bump]
];
let signer_seeds = &[&seeds[..]];


//mint
token::mint_to(CpiContext::new_with_signer(
    ctx.accounts.token_program.to_account_info(),
    MintTo {
        mint: ctx.accounts.lp_token_mint.to_account_info(),
        to: ctx.accounts.user_lp_token_ata.to_account_info(),
        authority: ctx.accounts.pool_state_account.to_account_info(),
    },
    signer_seeds // Pass signer seeds here
),
lp_tokens_to_mint // Mint LP tokens, not amount_a!
)?;


    Ok(())
}
