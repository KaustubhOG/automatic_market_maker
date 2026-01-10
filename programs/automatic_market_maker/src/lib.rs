pub mod error;
pub mod helper;
pub mod instructions;
pub mod state;

pub use anchor_lang::prelude::*;
pub use error::*;
pub use helper::*;
pub use instructions::*;

declare_id!("3ZrN6UmFKvZbkuEwJUBGV1RxbGgeXDzu2UPLj48PPmfk");

#[program]
pub mod automatic_market_maker {
    use super::*;

    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        token_a: Pubkey,
        token_b: Pubkey,
    ) -> Result<()> {
        instructions::initialize_pool::handler(ctx, token_a, token_b)?;
        Ok(())
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
        instructions::add_liquidity::handler(ctx, amount_a, amount_b)?;
        Ok(())
    }
    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>, amount_lp: u64) -> Result<()> {
        instructions::remove_liquidity::handler(ctx, amount_lp)?;
        Ok(())
    }
    pub fn swap(
        ctx: Context<Swap>,
        amount_to_swap: u64,
        minimum_amoun_to_get_after_swap: u64,
        swap_a_to_b: bool 
    ) -> Result<()> {
        instructions::swap::handler(ctx, amount_to_swap, minimum_amoun_to_get_after_swap,swap_a_to_b)?;
        Ok(())
    }
}
