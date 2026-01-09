use crate::state::*;
use crate::error::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer_checked, TransferChecked};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, burn, Burn};

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
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

pub fn handler(ctx:Context<RemoveLiquidity>,amount_lp:u64) -> Result<()> {
    require!(amount_lp>0,AmmErrors::INCORRECT);



    let reserve_a = ctx.accounts.vault_token_a.amount;
    let reserve_b = ctx.accounts.vault_token_b.amount;
    let total_lp_supply = ctx.accounts.lp_token_mint.supply;
     
    let amount_a = (amount_lp * reserve_a) / total_lp_supply;
    let amount_b = (amount_lp * reserve_b) / total_lp_supply;

    require!(amount_a > 0 && amount_b > 0, AmmErrors::INCORRECT);

    let token_a_key = ctx.accounts.token_a_mint.key();
    let token_b_key = ctx.accounts.token_b_mint.key();

    let seeds = &[
        b"pool_state_account".as_ref(),
        token_a_key.as_ref(),
        token_b_key.as_ref(),
        &[ctx.accounts.pool_state_account.bump]
      ];
    let signer_seeds = &[&seeds[..]];


//burn
burn(CpiContext::new(ctx.accounts.token_program.to_account_info(), Burn{
    mint : ctx.accounts.lp_token_mint.to_account_info(),
    from:ctx.accounts.user_lp_token_ata.to_account_info(),
    authority:ctx.accounts.user.to_account_info()
}),amount_lp)?;


//transfer a 
transfer_checked(CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), TransferChecked{
    from:ctx.accounts.vault_token_a.to_account_info(),
    mint:ctx.accounts.token_a_mint.to_account_info(),
    to:ctx.accounts.user_token_a_ata.to_account_info(),
    authority:ctx.accounts.pool_state_account.to_account_info()

}, signer_seeds),amount_a,ctx.accounts.token_a_mint.decimals)?;

//transfer b
transfer_checked(CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), TransferChecked{
    from:ctx.accounts.vault_token_b.to_account_info(),
    mint:ctx.accounts.token_b_mint.to_account_info(),
    to:ctx.accounts.user_token_b_ata.to_account_info(),
    authority:ctx.accounts.pool_state_account.to_account_info()

}, signer_seeds),amount_b,ctx.accounts.token_b_mint.decimals)?;

 
    Ok(())
}
