pub mod error;
pub mod instructions;
pub mod state;

pub use anchor_lang::prelude::*;
pub use error::*;
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
    // pub fn remove_liquidity(ctx: Context<RemoveLiquidity>) -> Result<()> {
    //     instructions::remove_liquidity::handler()?;
    //     Ok(())
    // }
    // pub fn swap(ctx: Context<Swap>) -> Result<()> {
    //     instructions::swap::handler()?;
    //     Ok(())
    // }
}
