use crate::state::*;
use crate::helper::sqrt::sqrt;
use crate::error::AmmErrors;
use anchor_spl::token::{};
use std::cmp::min;
use anchor_lang::prelude::*;
use anchor_spl::{token::{}, token_interface::{Mint, TokenAccount, TokenInterface}};

#[derive(Accounts)]
pub struct Swap <'a>{
    
}

pub fn handler(
    ctx: Context<Swap>,
    amount_to_swap: u64,
    minimum_amoun_to_get_after_swap: u64,
    swap_a_to_b: bool 
) -> Result<()> {
    Ok(())
}
