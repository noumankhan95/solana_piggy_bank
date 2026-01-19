use anchor_lang::prelude::*;

use crate::state::PiggyBank;
use crate::errors::PiggyBankError;
#[derive(Accounts)]
pub struct Unlock<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = 
            [b"piggy_bank",
            signer.key().as_ref()]
      ,
        bump = piggy_bank.bump
    )]
    pub piggy_bank: Account<'info, PiggyBank>,
    pub system_program: Program<'info, System>,
}

pub fn unlock_solana(ctx:Context<Unlock>,amount:u64)->Result<()>{
    let piggy_bank = &mut ctx.accounts.piggy_bank;
    require_keys_eq!(piggy_bank.owner,ctx.accounts.signer.key(),PiggyBankError::Unauthorized);
    require!(piggy_bank.unlock_time < Clock::get()?.unix_timestamp,PiggyBankError::TooEarly);
    require!(amount <= piggy_bank.amount, PiggyBankError::InsufficientFunds);
    let cpi_accounts = anchor_lang::system_program::Transfer {
        from: piggy_bank.to_account_info(),
        to: ctx.accounts.signer.to_account_info(),
    };
   let seeds = &[
        b"piggy_bank",
        ctx.accounts.signer.key.as_ref(),
        &[piggy_bank.bump],
    ];
    let signer = &[&seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.system_program.to_account_info(), cpi_accounts, signer);
    anchor_lang::system_program::transfer(cpi_ctx, amount)?;

    piggy_bank.amount = 0;
    Ok(())
}