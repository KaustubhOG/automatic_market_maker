use anchor_lang::prelude::*;
pub mod instructions;
use instructions::*;
declare_id!("3ZrN6UmFKvZbkuEwJUBGV1RxbGgeXDzu2UPLj48PPmfk");

#[program]
pub mod automatic_market_maker {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}


