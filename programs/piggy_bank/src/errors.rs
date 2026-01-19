use anchor_lang::prelude::*;
#[error_code]
pub enum PiggyBankError {
    #[msg("You are not authorized.")]
    Unauthorized,
    #[msg("Cannot withdraw yet. Lock period not finished.")]
    TooEarly,
    #[msg("Account Doesnt have enough funds")]
    InsufficientFunds,
}
