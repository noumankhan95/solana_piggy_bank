use anchor_lang::prelude::*;

use crate::state::PiggyBank;

#[derive(Accounts)]
pub struct CreatePiggyBank<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init,payer=signer,space=8+PiggyBank::INIT_SPACE,seeds=[b"piggy_bank",signer.key().as_ref()],bump)]
    pub piggy_bank: Account<'info, PiggyBank>,
    pub system_program: Program<'info, System>,
}

pub fn create_piggy_bank(ctx: Context<CreatePiggyBank>, lock_time: i64) -> Result<()> {
    ctx.accounts.piggy_bank.owner = ctx.accounts.signer.key();
    ctx.accounts.piggy_bank.amount = 0;
    ctx.accounts.piggy_bank.unlock_time = Clock::get()?.unix_timestamp + lock_time;
    Ok(())
}
