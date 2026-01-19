use anchor_lang::prelude::*;

use crate::errors::PiggyBankError;
use crate::state::PiggyBank;
#[derive(Accounts)]
pub struct Lock<'info> {
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [
            b"piggy_bank",
            signer.key().as_ref()
        ],
        bump = piggy_bank.bump
    )]
    pub piggy_bank: Account<'info, PiggyBank>,
    pub system_program: Program<'info, System>,
}

pub fn lock_solana(ctx: Context<Lock>, amount: u64) -> Result<()> {
    let piggy_bank = &mut ctx.accounts.piggy_bank;

    require_keys_eq!(
        piggy_bank.owner,
        ctx.accounts.signer.key(),
        PiggyBankError::Unauthorized
    );

    let cpi_accounts = anchor_lang::system_program::Transfer {
        from: ctx.accounts.signer.to_account_info(),
        to: piggy_bank.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
    anchor_lang::system_program::transfer(cpi_ctx, amount)?;
    piggy_bank.amount += amount;
    Ok(())
}
