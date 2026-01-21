use anchor_lang::prelude::*;

use crate::errors::PiggyBankError;
use crate::state::PiggyBank;
#[derive(Accounts)]
pub struct Lock<'info> {
    #[account(mut)]
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

    // Build transfer instruction: from signer -> PDA
    let ix = system_instruction::transfer(&ctx.accounts.signer.key(), &piggy_bank.key(), amount);

    // Execute transfer
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.signer.to_account_info(),
            piggy_bank.to_account_info(),
        ],
    )?;

    // Update PiggyBank account state
    piggy_bank.amount += amount;

    Ok(())
}
