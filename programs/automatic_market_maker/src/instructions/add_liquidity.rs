use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct AddLiquidity {}
pub fn handler(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
    Ok(())
}
