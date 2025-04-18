use anchor_lang::prelude::*;

#[account]
pub struct ApproveSigner {
    pub approve_signer: [u8; 32],
}
