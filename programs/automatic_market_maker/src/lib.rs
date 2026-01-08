use anchor_lang::prelude::*;
pub mod instructions;
use instructions::*;
declare_id!("3ZrN6UmFKvZbkuEwJUBGV1RxbGgeXDzu2UPLj48PPmfk");

#[program]
pub mod automatic_market_maker {
    use super::*;

    pub fn initialize_pool() -> Result<()> {
        Ok(())
    }
    pub fn add_liquidity() -> Result<()> {
        Ok(())
    }
    pub fn remove_liquidity() -> Result<()> {
        Ok(())
    }
    pub fn swap() -> Result<()> {
        Ok(())
    }
}
