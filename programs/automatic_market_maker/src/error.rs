use anchor_lang::prelude::*;

#[error_code]
pub enum AmmErrors {
    #[msg("error 2")]
    INCORRECT,
}
