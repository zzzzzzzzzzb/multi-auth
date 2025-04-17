use anchor_lang::prelude::*;

#[account]
pub struct ApproveSigner {
    pub owner: Pubkey,
    pub approve_signer: Pubkey,
}
