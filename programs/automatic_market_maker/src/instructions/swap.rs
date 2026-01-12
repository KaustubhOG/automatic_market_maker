use crate::state::*;
use crate::helper::sqrt::sqrt;
use crate::error::AmmErrors;
use anchor_spl::token::{TransferChecked, transfer_checked};
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
 require!(amount_to_swap>0, AmmErrors::INCORRECT);
  require!(minimum_amoun_to_get_after_swap>0, AmmErrors::INCORRECT);

//we have to change the direction from where swap is happening 


let (vault_in, vault_out, user_ata_in, user_ata_out, mint_in, mint_out) = if swap_a_to_b {
    // Swapping A → B
    (
        &ctx.accounts.token_a_mint_vault_ata,  
        &ctx.accounts.token_b_mint_vault_ata,
        &ctx.accounts.token_a_mint_user_ata,
        &ctx.accounts.token_b_mint_user_ata,  
        &ctx.accounts.token_a_mint,
        &ctx.accounts.token_b_mint,
    )
} else {
    // Swapping B → A
    (
        &ctx.accounts.token_b_mint_vault_ata,  
        &ctx.accounts.token_a_mint_vault_ata,
        &ctx.accounts.token_b_mint_user_ata,
        &ctx.accounts.token_a_mint_user_ata,  
        &ctx.accounts.token_b_mint,
        &ctx.accounts.token_a_mint,
    )
};



// a * b = k

//  lets suppose the swap happning a->b  so new.reserve.a * new.reserve.b = k 
// and the amount we have to given amount_out is new.reserve.b-old.reserve.b 
// simply it we will get the overall idea
let reserve_in = vault_in.amount;
let reserve_out = vault_out.amount;

let amount_out = (amount_to_swap * reserve_out) / (reserve_in + amount_to_swap);

require!(amount_out>=minimum_amoun_to_get_after_swap,AmmErrors::INCORRECT);

//user ata -> vault 
transfer_checked(CpiContext::new(ctx.accounts.token_program.to_account_info(),TransferChecked{
    from : user_ata_in.to_account_info(),
    mint:mint_in.to_account_info(),
    to:vault_in.to_account_info(),
    authority:ctx.accounts.user.to_account_info()
}), amount_to_swap, mint_in.decimals)?;

let token_a_mint=ctx.accounts.token_a_mint.key();
let token_b_mint=ctx.accounts.token_b_mint.key();


let seeds =&[b"pool_state_account",token_a_mint.as_ref(),token_b_mint.as_ref(),&[ctx.accounts.pool_state_account.bump]];
let signer_seeds=&[&seeds[..]];

//vault se user pe 
transfer_checked(CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), TransferChecked{
    from : vault_out.to_account_info(),
    mint:mint_out.to_account_info(),
    to:user_ata_out.to_account_info(),
    authority:ctx.accounts.pool_state_account.to_account_info()
}, signer_seeds), amount_out, mint_out.decimals)?;


    Ok(())
}
