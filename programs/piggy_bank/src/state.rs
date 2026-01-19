use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PiggyBank {
    pub owner: Pubkey,
    pub amount: u64,
    pub unlock_time: i64,
    pub bump: u8,
}
