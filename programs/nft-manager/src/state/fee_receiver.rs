use anchor_lang::prelude::*;

#[account]
pub struct FeeReceiver {
    pub receiver: Pubkey,
    pub height: u64,
}
