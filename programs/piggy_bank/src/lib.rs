use anchor_lang::prelude::*;
mod errors;
mod instructions;
mod state;
declare_id!("26idqGYy3q6ujrkBuL5ugKu3j4dXTZD7KswEmg5NHJYq");

#[program]
pub mod piggy_bank {

    use super::*;
    use instructions::*;
    pub fn initialize(ctx: Context<CreatePiggyBank>, lock_time: i64) -> Result<()> {
        create_piggy_bank(ctx, lock_time)?;
        Ok(())
    }

    pub fn lock_funds(ctx: Context<Lock>, amount: u64) -> Result<()> {
        lock_solana(ctx, amount)?;
        Ok(())
    }

    pub fn unlock_funds(ctx: Context<Unlock>, amount: u64) -> Result<()> {
        unlock_solana(ctx, amount)?;
        Ok(())
    }
}
