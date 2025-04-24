use anchor_lang::prelude::*;

#[account]
pub struct AuthedSigner {
    pub authed_signer: [u8; 32],
}
